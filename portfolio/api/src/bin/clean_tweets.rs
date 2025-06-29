use anyhow::Result;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::{Client, Collection};
use std::env;

async fn clean_tweets(collection: &Collection<Document>) -> Result<()> {
    let mut cursor = collection.find(doc! {}).await?;

    while let Some(result) = cursor.try_next().await? {
        let doc = result;

        let title = doc.get_str("title").unwrap_or_default();
        let clean_title = sanitize_string(title);

        let description = doc.get_str("description").unwrap_or_default();
        let clean_description = sanitize_string(description);

        let id = doc
            .get_object_id("_id")
            .map_err(|_| anyhow::anyhow!("Invalid _id"))?;

        collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": {
                    "title": clean_title,
                    "description": clean_description
                }},
            )
            .await?;
    }

    Ok(())
}

fn sanitize_string(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii() { c } else { ' ' })
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
    let client = Client::with_uri_str(&mongo_url).await?;
    let db = client.database("rss-bot");
    let collection = db.collection("tweets");

    clean_tweets(&collection).await?;

    Ok(())
}

// Tests d'intégration supprimés - nécessitent un environnement MongoDB configuré
