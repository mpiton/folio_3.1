use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::services::rss::FeedService;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    page: u32,
    #[serde(default = "default_limit")]
    limit: u32,
}

fn default_page() -> u32 {
    1
}

fn default_limit() -> u32 {
    9
}

pub async fn get_feeds(
    State(feed_service): State<Arc<FeedService>>,
    Query(params): Query<PaginationParams>,
) -> Json<Vec<crate::models::rss::RssItem>> {
    let feeds = feed_service.get_feeds(params.page, params.limit).await;
    Json(feeds)
}
