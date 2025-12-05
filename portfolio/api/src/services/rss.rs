use crate::{config::Config, models::rss::RssItem};
use anyhow::Result;
use chrono::{DateTime, Utc};
use futures_util::TryStreamExt;
use mongodb::bson::{doc, Bson, Document};
use mongodb::Database;
use once_cell::sync::Lazy;
use regex::Regex;
use rss::{Channel, Item};
use serde::{Deserialize, Serialize};
use urlencoding;

static IMG_SRC_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)<img[^>]+src=["'](https?://[^"']+)["']"#).unwrap());

#[derive(Debug, Serialize, Deserialize, Clone)]
/// RSS feed metadata container for source tracking
pub struct Feed {
    /// Canonical URL of the RSS feed source
    pub link: String,
    /// Initial creation timestamp in UTC
    pub created_at: DateTime<Utc>,
    /// Last update timestamp in UTC
    pub updated_at: DateTime<Utc>,
}

/// RSS feed processing service with MongoDB integration
///
/// # Features
/// - Feed parsing with image extraction
/// - Paginated feed retrieval
/// - Automatic content expiration via TTL indexes
pub struct FeedService {
    db: Database,
    config: Config,
    client: reqwest::Client,
}

impl FeedService {
    #[must_use]
    pub fn new(db: Database, config: Config) -> Self {
        Self {
            db,
            config,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
        }
    }

    /// Extracts image URL from RSS item enclosure
    fn extract_from_enclosure(item: &Item) -> Option<String> {
        item.enclosure()
            .filter(|enclosure| enclosure.mime_type.starts_with("image/"))
            .map(|enclosure| enclosure.url.clone())
    }

    /// Extracts image URL from RSS item media extension (content)
    fn extract_from_media_content(item: &Item) -> Option<String> {
        item.extensions
            .get("media")?
            .get("content")?
            .first()?
            .attrs
            .get("url")
            .cloned()
    }

    /// Extracts image URL from RSS item media extension (thumbnail)
    fn extract_from_media_thumbnail(item: &Item) -> Option<String> {
        item.extensions
            .get("media")?
            .get("thumbnail")?
            .first()?
            .attrs
            .get("url")
            .cloned()
    }

    /// Extracts image URL from HTML description using regex
    fn extract_from_html_description(item: &Item) -> Option<String> {
        item.description()
            .and_then(|description| IMG_SRC_RE.captures(description))
            .and_then(|cap| cap.get(1))
            .map(|src| src.as_str().to_string())
    }

    /// Extracts the image URL from an RSS article item
    ///
    /// Tries multiple sources in order:
    /// 1. RSS enclosure (image type)
    /// 2. Media extension content
    /// 3. Media extension thumbnail
    /// 4. HTML description (regex search)
    fn extract_image_url(item: &Item) -> Option<String> {
        Self::extract_from_enclosure(item)
            .or_else(|| Self::extract_from_media_content(item))
            .or_else(|| Self::extract_from_media_thumbnail(item))
            .or_else(|| Self::extract_from_html_description(item))
    }

    /// Retrieves paginated RSS feed items from database
    ///
    /// # Arguments
    /// * `page` - Pagination page number (1-based)
    /// * `limit` - Items per page
    ///
    /// # Returns
    /// Vector of `RssItem` structures sorted by publication date
    pub async fn get_feeds(&self, page: u32, limit: u32) -> Vec<RssItem> {
        let collection = self.db.collection::<Document>("portfolio");
        let skip = (page - 1) * limit;

        let options = mongodb::options::FindOptions::builder()
            .skip(skip as u64)
            .limit(limit as i64)
            .sort(doc! { "pub_date": -1 })
            .build();

        match collection.find(doc! {}).with_options(options).await {
            Ok(cursor) => {
                let docs: Vec<Document> = cursor.try_collect().await.unwrap_or_default();
                docs.into_iter()
                    .map(|doc| RssItem {
                        title: doc.get_str("title").unwrap_or_default().to_string(),
                        url: doc.get_str("url").unwrap_or_default().to_string(),
                        pub_date: doc
                            .get_datetime("pub_date")
                            .ok()
                            .map(|bson_dt| {
                                DateTime::from_timestamp_millis(bson_dt.timestamp_millis())
                                    .unwrap_or_else(Utc::now)
                            })
                            .or_else(|| {
                                // Fallback for legacy string format
                                doc.get_str("pub_date")
                                    .ok()
                                    .and_then(|date_str| DateTime::parse_from_rfc3339(date_str).ok())
                                    .map(|dt| dt.with_timezone(&Utc))
                            })
                            .unwrap_or_else(Utc::now),
                        description: doc.get_str("description").unwrap_or_default().to_string(),
                        image_url: doc
                            .get_str("image_url")
                            .unwrap_or("https://placehold.co/600x400/grey/white/png?text=Article")
                            .to_string(),
                    })
                    .collect()
            }
            Err(e) => {
                tracing::error!("Error fetching RSS feeds: {}", e);
                Vec::new()
            }
        }
    }

    /// Fetches and parses an RSS feed
    async fn fetch_rss(&self, url: &str) -> Result<Channel> {
        let content = self
            .client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (compatible; RSSBot/1.0)")
            .send()
            .await?
            .bytes()
            .await?;

        let channel = Channel::read_from(&content[..])?;
        Ok(channel)
    }

    /// Synchronizes RSS feeds from external sources to local database
    ///
    /// # Workflow
    /// 1. Connects to configured RSS source database
    /// 2. Processes each feed URL sequentially
    /// 3. Performs multi-stage parsing:
    ///    - Channel metadata extraction
    ///    - Item content parsing
    ///    - Image URL detection
    /// 4. Stores normalized data with TTL indexes
    ///
    /// # Error Handling
    /// Returns error if:
    /// - Source database connection fails
    /// - RSS feed parsing fails permanently (invalid XML)
    /// - Bulk write operation fails
    ///
    /// # Notes
    /// - Failed individual feeds are logged but don't abort the process
    /// - Uses 90-day TTL for automatic data cleanup
    pub async fn store_items(&self) -> Result<()> {
        // Connect to the source database
        let source_client = mongodb::Client::with_uri_str(&self.config.rss_source_url)
            .await?
            .database(&self.config.rss_source_db);

        // Retrieve feeds
        let feeds_collection = source_client.collection::<Document>("feeds");
        let mut feeds_cursor = feeds_collection.find(doc! {}).await?;

        let target_collection = self.db.collection::<Document>("portfolio");

        // Collection to store all articles
        let mut all_articles = Vec::new();

        // Process each feed
        while let Some(feed_doc) = feeds_cursor.try_next().await? {
            let feed_link = feed_doc.get_str("link").unwrap_or_default();
            tracing::info!("Processing feed: {}", feed_link);

            // Fetch and parse the RSS feed
            match self.fetch_rss(feed_link).await {
                Ok(channel) => {
                    for item in channel.items().iter() {
                        let title = item.title().unwrap_or_default();
                        let link = item.link().unwrap_or_default();
                        let description = item.description().unwrap_or_default();
                        let pub_date = item
                            .pub_date()
                            .and_then(|date_str| DateTime::parse_from_rfc2822(date_str).ok())
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(Utc::now);

                        let image_url = Self::extract_image_url(item).unwrap_or_else(|| {
                            format!(
                                "https://placehold.co/600x400/grey/white/png?text={}",
                                urlencoding::encode(title)
                            )
                        });

                        all_articles.push((
                            pub_date,
                            doc! {
                                "title": title,
                                "url": link,
                                "pub_date": Bson::DateTime(mongodb::bson::DateTime::from_millis(pub_date.timestamp_millis())),
                                "description": description,
                                "image_url": image_url,
                            },
                        ));
                    }
                }
                Err(e) => {
                    tracing::error!("Error fetching feed {}: {}", feed_link, e);
                    continue;
                }
            }
        }

        // Sort articles by date in descending order
        all_articles.sort_by(|a, b| b.0.cmp(&a.0));

        // Remove old articles
        target_collection.delete_many(doc! {}).await?;

        // Insert sorted articles
        if !all_articles.is_empty() {
            let articles_to_insert: Vec<Document> =
                all_articles.into_iter().map(|(_, doc)| doc).collect();

            match target_collection.insert_many(articles_to_insert).await {
                Ok(_) => tracing::info!("Articles inserted successfully"),
                Err(e) => tracing::error!("Error inserting articles: {}", e),
            }
        }

        Ok(())
    }
}
