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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_clean_tweets() {
        std::env::set_var("DOTENV_FILE", ".env.test");
        dotenv::from_filename(".env.test").ok();

        let base_mongo_url = std::env::var("MONGO_URL").expect("MONGO_URL must be set");
        let mongo_db = std::env::var("MONGO_DB").expect("MONGO_DB must be set");
        let mongo_url = format!("{}?authSource={}", base_mongo_url, mongo_db);

        // Arrange
        let client = Client::with_uri_str(&mongo_url).await.unwrap();
        println!("Connected to MongoDB");

        let db = client.database("portfolio_test");
        let collection = db.collection("tweets_test");
        println!("Got collection reference");

        // Nettoyer la collection avant le test
        let _ = collection.drop().await;
        println!("Dropped collection (if it existed)");

        let tweets = vec![
            doc! { "title": "Titre 1", "description": "Description 1" },
            doc! { "title": "Titre 2 avec caractères bizarres ¢©ß", "description": "Description 2 ~ok" },
            doc! { "title": "Titre 3", "description": "Description 3 \u{1F600}" },
        ];

        println!("Inserting {} tweets...", tweets.len());
        match collection.insert_many(tweets).await {
            Ok(_) => println!("Successfully inserted tweets"),
            Err(e) => panic!("Failed to insert tweets: {e}"),
        }

        // Act
        println!("Running clean_tweets...");
        clean_tweets(&collection).await.unwrap();
        println!("Finished clean_tweets");

        // Assert
        println!("Fetching cleaned tweets...");
        let mut cleaned_tweets = collection.find(doc! {}).await.unwrap();
        let mut count = 0;

        // Vérification des résultats
        let mut titles = Vec::new();
        let mut descriptions = Vec::new();
        while let Some(tweet) = cleaned_tweets.try_next().await.unwrap() {
            count += 1;
            titles.push(tweet.get_str("title").unwrap().to_string());
            descriptions.push(tweet.get_str("description").unwrap().to_string());
        }

        println!("Found {} cleaned tweets", count);

        // Vérification silencieuse des résultats
        assert!(!titles.is_empty(), "Les titres ne devraient pas être vides");
        assert!(
            !descriptions.is_empty(),
            "Les descriptions ne devraient pas être vides"
        );

        // Nettoyer après le test
        let _ = collection.drop().await;
        println!("Cleaned up test collection");
    }
}
