use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RssFeed {
    pub title: String,
    pub link: String,
    pub description: String,
    pub published_at: DateTime<Utc>,
}
