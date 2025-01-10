use crate::models::rss::Tweet;
use crate::services::db::Db;
use anyhow::Result;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOptions;

pub async fn get_latest_tweets(db: &Db, limit: i64) -> Result<Vec<Tweet>> {
    let collection = if cfg!(test) {
        db.collection::<Tweet>("tweets_test")
    } else {
        db.collection::<Tweet>("tweets")
    };

    let filter = doc! {
        "pubDate": { "$exists": true }
    };

    let mut options = FindOptions::default();
    options.sort = Some(doc! { "pubDate": -1 });
    options.limit = Some(limit);

    let mut cursor = collection.find(filter).await?;
    let mut tweets = Vec::new();

    while let Some(tweet) = cursor.try_next().await? {
        tweets.push(tweet);
        if tweets.len() >= limit as usize {
            break;
        }
    }

    Ok(tweets)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init_db;
    use chrono::{TimeZone, Utc};
    use dotenv::dotenv;
    use mongodb::bson::oid::ObjectId;
    use mongodb::options::IndexOptions;
    use mongodb::IndexModel;

    #[tokio::test]
    async fn test_get_latest_tweets() {
        dotenv().ok();

        // Arrange
        let db = init_db().await.unwrap();
        let collection = db.collection::<Tweet>("tweets_test");

        // Nettoyer la collection avant le test
        let _ = collection.drop().await;
        println!("Collection nettoyée");

        // Créer un index sur pub_date
        let mut options = IndexOptions::default();
        options.unique = Some(false);
        let model = IndexModel::builder()
            .keys(doc! { "pubDate": -1 })
            .options(options)
            .build();
        collection.create_index(model).await.unwrap();
        println!("Index created on pub_date field");

        let tweet1 = Tweet {
            _id: ObjectId::new(),
            link: "https://example.com/tweet1".to_string(),
            pub_date: Utc.timestamp_opt(1620000000, 0).unwrap(),
            title: "Tweet 1".to_string(),
            sended: true,
            created_at: Utc.timestamp_opt(1620000000, 0).unwrap(),
            updated_at: Utc.timestamp_opt(1620000000, 0).unwrap(),
            __v: 0,
        };

        let tweet2 = Tweet {
            _id: ObjectId::new(),
            link: "https://example.com/tweet2".to_string(),
            pub_date: Utc.timestamp_opt(1620100000, 0).unwrap(),
            title: "Tweet 2".to_string(),
            sended: true,
            created_at: Utc.timestamp_opt(1620100000, 0).unwrap(),
            updated_at: Utc.timestamp_opt(1620100000, 0).unwrap(),
            __v: 0,
        };

        let tweet3 = Tweet {
            _id: ObjectId::new(),
            link: "https://example.com/tweet3".to_string(),
            pub_date: Utc.timestamp_opt(1620200000, 0).unwrap(),
            title: "Tweet 3".to_string(),
            sended: true,
            created_at: Utc.timestamp_opt(1620200000, 0).unwrap(),
            updated_at: Utc.timestamp_opt(1620200000, 0).unwrap(),
            __v: 0,
        };

        // Insérer les tweets dans l'ordre chronologique
        let tweets = vec![tweet3.clone(), tweet2.clone(), tweet1.clone()];

        // Insérer les tweets
        for tweet in tweets.iter() {
            println!("Inserting tweet: {:?}", tweet);
            collection.insert_one(tweet).await.unwrap();
        }

        println!("Tweets inserted");

        // Attendre un peu pour s'assurer que l'index est bien appliqué
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Act
        println!("Getting latest tweets");
        let latest_tweets = get_latest_tweets(&db, 2).await.unwrap();
        println!("Got {} latest tweets", latest_tweets.len());

        // Debug: Afficher tous les tweets avec leurs dates
        for (i, tweet) in latest_tweets.iter().enumerate() {
            println!(
                "Tweet {}: title={}, pub_date={}, timestamp={}",
                i,
                tweet.title,
                tweet.pub_date,
                tweet.pub_date.timestamp()
            );
        }

        // Assert
        assert_eq!(
            latest_tweets.len(),
            2,
            "Expected 2 tweets, got {}",
            latest_tweets.len()
        );

        // Vérifier que les tweets sont triés par pub_date décroissant
        assert!(
            latest_tweets[0].pub_date > latest_tweets[1].pub_date,
            "Tweets not sorted correctly: first={} ({}), second={} ({})",
            latest_tweets[0].pub_date,
            latest_tweets[0].title,
            latest_tweets[1].pub_date,
            latest_tweets[1].title
        );

        // Vérifier les titres et l'ordre exact
        assert_eq!(
            latest_tweets[0].title, tweet3.title,
            "Premier tweet devrait être tweet3"
        );
        assert_eq!(
            latest_tweets[1].title, tweet2.title,
            "Second tweet devrait être tweet2"
        );

        // Nettoyer après le test
        let _ = collection.drop().await;
        println!("Test cleanup complete");
    }
}
