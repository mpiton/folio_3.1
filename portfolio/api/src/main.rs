use axum::{routing::get, Router};
use portfolio_api::{
    config::Config,
    routes::health::check,
    services::{db, rss::FeedService},
};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = Config::new();

    // Initialiser la base de données
    db::initialize()
        .await
        .expect("Failed to initialize database");

    let db = mongodb::Client::with_uri_str(&config.mongo_url)
        .await
        .expect("Failed to connect to MongoDB")
        .database("portfolio");

    let feed_service = Arc::new(FeedService::new(db, config));

    let app = Router::new()
        .route("/health", get(check))
        .with_state(feed_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on {addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
