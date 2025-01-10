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

        // Créer un index sur pub_date
        let mut options = IndexOptions::default();
        options.unique = Some(false);
        let model = IndexModel::builder()
            .keys(doc! { "pubDate": -1 })
            .options(options)
            .build();
        collection.create_index(model).await.unwrap();

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
            collection.insert_one(tweet).await.unwrap();
        }

        // Act
        let latest_tweets = get_latest_tweets(&db, 2).await.unwrap();

        // Assert
        assert_eq!(latest_tweets.len(), 2, "Expected 2 tweets");

        // Vérifier que les tweets sont triés par pub_date décroissant
        assert!(
            latest_tweets[0].pub_date > latest_tweets[1].pub_date,
            "Tweets not sorted correctly"
        );

        // Vérifier les titres et l'ordre exact
        assert_eq!(latest_tweets[0].title, tweet3.title);
        assert_eq!(latest_tweets[1].title, tweet2.title);

        // Nettoyer après le test
        let _ = collection.drop().await;
    }
}
