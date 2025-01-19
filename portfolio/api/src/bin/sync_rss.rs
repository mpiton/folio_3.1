use anyhow::Result;
use portfolio_api::{
    config::Config,
    services::{db, rss::FeedService},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::new();

    // Initialiser la base de données
    db::initialize().await?;

    let db = mongodb::Client::with_uri_str(&config.mongo_url)
        .await?
        .database("portfolio");

    let feed_service = FeedService::new(db, config);

    // Synchroniser les articles
    tracing::info!("Début de la synchronisation des articles RSS");
    feed_service.store_items().await?;
    tracing::info!("Synchronisation terminée avec succès");

    Ok(())
}
