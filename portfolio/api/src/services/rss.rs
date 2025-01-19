use crate::{config::Config, models::rss::RssItem};
use anyhow::Result;
use chrono::{DateTime, Utc};
use futures_util::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::Database;
use rss::{Channel, Item};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use urlencoding;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Feed {
    pub link: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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

    /// Extrait l'URL de l'image d'un article RSS
    fn extract_image_url(item: &Item) -> Option<String> {
        // 1. Vérifier les enclosures
        if let Some(enclosure) = item.enclosure() {
            if enclosure.mime_type.starts_with("image/") {
                return Some(enclosure.url.clone());
            }
        }

        // 2. Vérifier les extensions (media:content, media:thumbnail)
        if let Some(ext) = item.extensions.get("media") {
            if let Some(contents) = ext.get("content") {
                if let Some(content) = contents.first() {
                    if let Some(url) = content.attrs.get("url") {
                        return Some(url.clone());
                    }
                }
            }
            if let Some(thumbnails) = ext.get("thumbnail") {
                if let Some(thumbnail) = thumbnails.first() {
                    if let Some(url) = thumbnail.attrs.get("url") {
                        return Some(url.clone());
                    }
                }
            }
        }

        // 3. Chercher dans la description HTML
        if let Some(description) = item.description() {
            let fragment = Html::parse_fragment(description);
            if let Ok(selector) = Selector::parse("img") {
                if let Some(img) = fragment.select(&selector).next() {
                    if let Some(src) = img.value().attr("src") {
                        return Some(src.to_string());
                    }
                }
            }
        }

        None
    }

    /// Récupère tous les flux RSS de la base de données.
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
                            .get_str("pub_date")
                            .ok()
                            .and_then(|date_str| DateTime::parse_from_rfc3339(date_str).ok())
                            .map(|dt| dt.with_timezone(&Utc))
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

    /// Récupère et parse un flux RSS
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

    /// Stocke les éléments du flux RSS dans la base de données.
    ///
    /// # Errors
    ///
    /// Cette fonction retourne une erreur si :
    /// - La requête à la base de données échoue
    /// - L'insertion ou la mise à jour des documents échoue
    pub async fn store_items(&self) -> Result<()> {
        // Connexion à la base de données source
        let source_client = mongodb::Client::with_uri_str(&self.config.rss_source_url)
            .await?
            .database(&self.config.rss_source_db);

        // Récupération des feeds
        let feeds_collection = source_client.collection::<Document>("feeds");
        let mut feeds_cursor = feeds_collection.find(doc! {}).await?;

        let target_collection = self.db.collection::<Document>("portfolio");
        // Suppression des anciens articles
        target_collection.delete_many(doc! {}).await?;

        // Pour chaque feed
        while let Some(feed_doc) = feeds_cursor.try_next().await? {
            let feed_link = feed_doc.get_str("link").unwrap_or_default();
            tracing::info!("Traitement du feed : {}", feed_link);

            // Récupération et parsing du flux RSS
            match self.fetch_rss(feed_link).await {
                Ok(channel) => {
                    for item in channel.items().iter().take(10) {
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

                        let new_doc = doc! {
                            "title": title,
                            "url": link,
                            "pub_date": pub_date.to_rfc3339(),
                            "description": description,
                            "image_url": image_url,
                        };

                        if let Err(e) = target_collection.insert_one(new_doc).await {
                            tracing::error!(
                                "Erreur lors de l'insertion de l'article '{}' : {}",
                                title,
                                e
                            );
                            continue;
                        }
                        tracing::info!("Article ajouté : {}", title);
                    }
                }
                Err(e) => {
                    tracing::error!(
                        "Erreur lors de la récupération du feed {} : {}",
                        feed_link,
                        e
                    );
                    continue;
                }
            }
        }

        Ok(())
    }
}
