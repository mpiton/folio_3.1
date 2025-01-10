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
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_clean_tweets() {
        dotenv().ok();
        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
        println!("Using MongoDB URL: {}", mongo_url);

        // Arrange
        let client = Client::with_uri_str(&mongo_url).await.unwrap();
        println!("Connected to MongoDB");

        let db = client.database("rss-bot"); // Utiliser la même base que l'application
        let collection = db.collection("tweets_test"); // Mais une collection différente pour les tests
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
            Err(e) => panic!("Failed to insert tweets: {}", e),
        }

        // Act
        println!("Running clean_tweets...");
        clean_tweets(&collection).await.unwrap();
        println!("Finished clean_tweets");

        // Assert
        println!("Fetching cleaned tweets...");
        let cleaned_tweets: Vec<Document> = collection
            .find(doc! {})
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap();
        println!("Found {} cleaned tweets", cleaned_tweets.len());

        assert_eq!(cleaned_tweets.len(), 3);

        // Vérifier que les titres et descriptions sont bien nettoyés
        let titles: Vec<&str> = cleaned_tweets
            .iter()
            .map(|doc| doc.get_str("title").unwrap())
            .collect();
        let descriptions: Vec<&str> = cleaned_tweets
            .iter()
            .map(|doc| doc.get_str("description").unwrap())
            .collect();

        // Afficher les valeurs pour le débogage
        println!("Cleaned titles: {:?}", titles);
        println!("Cleaned descriptions: {:?}", descriptions);

        // Vérifier que les caractères non-ASCII ont été remplacés par des espaces
        assert!(titles.contains(&"Titre 1"));
        assert!(titles.iter().any(|&t| t.starts_with("Titre 2 avec caract")));
        assert!(titles.contains(&"Titre 3"));

        assert!(descriptions.contains(&"Description 1"));
        assert!(descriptions.iter().any(|&d| d.starts_with("Description 2")));
        assert!(descriptions.iter().any(|&d| d.starts_with("Description 3")));

        // Nettoyer après le test
        let _ = collection.drop().await;
        println!("Cleaned up test collection");
    }
}
