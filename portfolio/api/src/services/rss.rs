use crate::config::Config;
use anyhow::Result;
use chrono::{DateTime, Utc};
use mongodb::bson::{doc, Document};
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeedItem {
    pub title: String,
    pub url: String,
    pub pub_date: DateTime<Utc>,
    pub description: String,
}

pub struct FeedService {
    db: Database,
}

impl FeedService {
    #[must_use]
    pub fn new(db: Database, _config: Config) -> Self {
        Self { db }
    }

    /// Stocke les éléments du flux RSS dans la base de données.
    ///
    /// # Errors
    ///
    /// Cette fonction retourne une erreur si :
    /// - La requête à la base de données échoue
    /// - L'insertion ou la mise à jour des documents échoue
    pub async fn store_items(&self, url: &str, items: &[FeedItem]) -> Result<()> {
        let collection = self.db.collection::<Document>("portfolio");

        for item in items {
            let doc = doc! {
                "title": &item.title,
                "url": url,
                "pub_date": &item.pub_date.to_rfc3339(),
                "description": &item.description,
            };

            let update = doc! {
                "$set": &doc
            };

            // Vérifie d'abord si le document existe
            if collection.find_one(doc! { "url": url }).await?.is_none() {
                collection.insert_one(doc).await?;
            } else {
                collection.update_one(doc! { "url": url }, update).await?;
            }
        }

        Ok(())
    }
}
