use crate::models::rss::Tweet;
use mongodb::{Client, Collection, Database};

pub type Db = Database;

#[allow(dead_code)]
pub type TweetCollection = Collection<Tweet>;

pub async fn init_db() -> mongodb::error::Result<Db> {
    let mongo_url = std::env::var("MONGO_URL").expect("MONGO_URL must be set");
    println!("Connecting to MongoDB at {}", mongo_url);

    let client = Client::with_uri_str(&mongo_url).await?;
    let db = client.database("rss-bot");

    Ok(db)
}

#[allow(dead_code)]
pub fn get_tweet_collection(db: &Db) -> TweetCollection {
    db.collection("tweets")
}
