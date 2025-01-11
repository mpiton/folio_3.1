use crate::config::Config;
use anyhow::Result;
use chrono::{DateTime, Utc};
use mongodb::bson::{doc, Bson, Document};
use mongodb::Database;
use rss::Channel;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RssItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: DateTime<Utc>,
}

pub struct RssService {
    db: Database,
    config: Config,
}

impl RssService {
    pub fn new(db: Database, config: Config) -> Self {
        RssService { db, config }
    }

    pub async fn fetch_and_store_feed(&self, url: &str) -> Result<Vec<RssItem>> {
        // Vérifier si nous avons des données en cache
        if let Some(cached_items) = self.get_cached_items(url).await? {
            return Ok(cached_items);
        }

        // Si pas de cache, récupérer les données
        let response = reqwest::get(url).await?;
        println!("RSS response status: {}", response.status());
        let content = response.bytes().await?;
        println!("RSS content: {}", String::from_utf8_lossy(&content));

        let channel = Channel::read_from(&content[..])?;

        let items: Vec<RssItem> = channel
            .items()
            .iter()
            .map(|item| {
                let pub_date = item
                    .pub_date()
                    .and_then(|date| DateTime::parse_from_rfc2822(date).ok())
                    .map(|date| date.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now);

                RssItem {
                    title: item.title().unwrap_or_default().to_string(),
                    link: item.link().unwrap_or_default().to_string(),
                    description: item.description().unwrap_or_default().to_string(),
                    pub_date,
                }
            })
            .collect();

        println!("Parsed {} items from RSS feed", items.len());

        // Stocker en cache
        self.store_items(url, &items).await?;

        Ok(items)
    }

    async fn get_cached_items(&self, url: &str) -> Result<Option<Vec<RssItem>>> {
        let collection = self.db.collection::<Document>("portfolio");

        let now = Utc::now();
        let expiration = now - Duration::from_secs(self.config.rss_cache_duration);

        let filter = doc! {
            "url": url,
            "timestamp": { "$gt": Bson::DateTime(mongodb::bson::DateTime::from_millis(expiration.timestamp_millis())) }
        };

        println!("Checking cache with filter: {:?}", filter);

        if let Some(doc) = collection.find_one(filter).await? {
            println!("Found cached document: {:?}", doc);
            if let Some(items) = doc.get("items").and_then(|i| i.as_array()) {
                println!("Found {} items in cache", items.len());
                let rss_items: Vec<RssItem> = items
                    .iter()
                    .filter_map(|item| {
                        let doc = item.as_document()?;
                        let title = doc.get_str("title").ok()?.to_string();
                        let link = doc.get_str("link").ok()?.to_string();
                        let description = doc.get_str("description").ok()?.to_string();
                        let pub_date = doc
                            .get_str("pub_date")
                            .ok()
                            .and_then(|d| DateTime::parse_from_rfc3339(d).ok())
                            .map(|d| d.with_timezone(&Utc))?;

                        Some(RssItem {
                            title,
                            link,
                            description,
                            pub_date,
                        })
                    })
                    .collect();
                println!("Deserialized {} items from cache", rss_items.len());
                return Ok(Some(rss_items));
            }
        } else {
            println!("No cached document found");
        }

        Ok(None)
    }

    async fn store_items(&self, url: &str, items: &[RssItem]) -> Result<()> {
        let collection = self.db.collection::<Document>("portfolio");

        // Supprimer l'ancien cache
        let delete_result = collection.delete_one(doc! { "url": url }).await?;
        println!("Deleted {} old cache entries", delete_result.deleted_count);

        // Convertir les items en documents BSON
        let items_docs: Vec<Document> = items
            .iter()
            .map(|item| {
                doc! {
                    "title": &item.title,
                    "link": &item.link,
                    "description": &item.description,
                    "pub_date": item.pub_date.to_rfc3339()
                }
            })
            .collect();

        let now = Utc::now();
        let doc = doc! {
            "url": url,
            "items": items_docs,
            "timestamp": Bson::DateTime(mongodb::bson::DateTime::from_millis(now.timestamp_millis()))
        };

        println!("Storing document in cache: {:?}", doc);
        collection.insert_one(doc).await?;
        println!("Successfully stored items in cache");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::db::test_utils::create_test_db;
    use std::time::Duration;
    use tokio::time::timeout;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    const TEST_TIMEOUT: Duration = Duration::from_secs(60);

    #[tokio::test]
    async fn test_fetch_and_store_feed() {
        // Test avec timeout
        match timeout(TEST_TIMEOUT, async {
            // Configuration de test
            let config = Config::test_config();

            // Créer une base de données de test
            let db = create_test_db()
                .await
                .expect("Failed to create test database");

            // Créer un mock server avec réponse rapide
            let mock_server = MockServer::start().await;
            let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
            <rss version="2.0">
                <channel>
                    <title>Test Feed</title>
                    <link>http://example.com</link>
                    <description>Test Description</description>
                    <item>
                        <title>Test Item</title>
                        <link>http://example.com/item</link>
                        <description>Test Item Description</description>
                        <pubDate>Tue, 15 Nov 2023 12:00:00 GMT</pubDate>
                    </item>
                </channel>
            </rss>"#;

            Mock::given(method("GET"))
                .and(path("/feed"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_string(mock_feed)
                        .set_delay(Duration::from_millis(100))
                        .insert_header("content-type", "application/rss+xml"),
                )
                .expect(1)
                .mount(&mock_server)
                .await;

            // Créer le service RSS
            let rss_service = RssService::new(db.clone(), config);

            // Tester la récupération du flux
            let items = rss_service
                .fetch_and_store_feed(&format!("{}/feed", mock_server.uri()))
                .await
                .expect("Failed to fetch feed");

            assert_eq!(items.len(), 1, "Expected 1 item in the feed");
            assert_eq!(items[0].title, "Test Item");

            // Vérifier le cache avec un délai minimal
            tokio::time::sleep(Duration::from_millis(100)).await;
            let cached_items = rss_service
                .get_cached_items(&format!("{}/feed", mock_server.uri()))
                .await
                .expect("Failed to get cached items")
                .expect("No cached items found");

            assert_eq!(cached_items.len(), 1, "Expected 1 item in the cache");
            assert_eq!(cached_items[0].title, "Test Item");

            // Nettoyer
            db.drop().await.ok();
        })
        .await
        {
            Ok(_) => (),
            Err(_) => panic!(
                "Le test a dépassé le délai de {} secondes",
                TEST_TIMEOUT.as_secs()
            ),
        }
    }
}
