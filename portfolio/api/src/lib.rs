pub mod config;
pub mod models;
pub mod routes;
pub mod services;

use services::{contact::ContactService, rss::RssService};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub rss_service: Arc<RssService>,
    pub contact_service: Arc<ContactService>,
}

pub use config::Config;
pub use services::*;
