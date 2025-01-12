use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub link: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedItem {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub feed_id: ObjectId,
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedFeed {
    pub title: String,
    pub link: String,
    pub description: String,
    pub items: Vec<ParsedItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: Option<DateTime<Utc>>,
}
