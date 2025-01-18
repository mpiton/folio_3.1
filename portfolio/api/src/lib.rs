pub mod config;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;

use services::{contact::MessageService, rss::FeedService};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub feed_service: Arc<FeedService>,
    pub message_service: Arc<MessageService>,
}

pub use config::Config;
pub use models::contact::Request as ContactRequest;
pub use models::rss::FeedItem;
