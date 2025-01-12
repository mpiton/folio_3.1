use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::services::rss::{FeedItem, FeedService};

pub async fn get_rss_feed(State(state): State<Arc<FeedService>>, url: String) -> impl IntoResponse {
    match state.store_items(&url, &[]).await {
        Ok(()) => Json::<Vec<FeedItem>>(vec![]).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {e}"),
        )
            .into_response(),
    }
}
