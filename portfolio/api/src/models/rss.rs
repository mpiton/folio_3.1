use chrono::{DateTime, Utc};
use mongodb::bson;
use serde::{Deserialize, Deserializer, Serialize};

#[allow(dead_code)]
fn deserialize_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let bson = bson::Bson::deserialize(deserializer)?;
    bson::from_bson(bson).map_err(serde::de::Error::custom)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RssFeed {
    pub title: String,
    pub link: String,
    pub description: String,
    pub items: Vec<RssItem>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RssItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tweet {
    pub _id: mongodb::bson::oid::ObjectId,
    pub link: String,
    #[serde(rename = "pubDate")]
    pub pub_date: DateTime<Utc>,
    pub title: String,
    pub sended: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub __v: i32,
}
