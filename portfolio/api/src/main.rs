use axum::{
    http::{HeaderName, Method},
    routing::get,
    Router,
};
use portfolio_api::{
    config::Config,
    middleware::RateLimiter,
    routes::{contact::handle_message, health::check},
    services::{contact::MessageService, db, rss::FeedService},
};
use std::sync::Arc;
use std::{net::SocketAddr, time::Duration};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::new();
    tracing::info!("Starting server with config: {:?}", config);

    // Initialiser la base de données
    db::initialize()
        .await
        .expect("Failed to initialize database");

    let db = mongodb::Client::with_uri_str(&config.mongo_url)
        .await
        .expect("Failed to connect to MongoDB")
        .database("portfolio");

    // Configure CORS
    let frontend_url = config.frontend_url.trim_end_matches('/');
    let cors = CorsLayer::new()
        .allow_origin([
            frontend_url.parse().unwrap(),
            format!("{}/", frontend_url).parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
            HeaderName::from_static("x-requested-with"),
            HeaderName::from_static("accept"),
            HeaderName::from_static("origin"),
            HeaderName::from_static("referer"),
            HeaderName::from_static("user-agent"),
            HeaderName::from_static("sec-ch-ua"),
            HeaderName::from_static("sec-ch-ua-mobile"),
            HeaderName::from_static("sec-ch-ua-platform"),
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    tracing::info!(
        "CORS configured with frontend URLs: {} and {}/",
        frontend_url,
        frontend_url
    );

    // Configure rate limiting (100 requests per minute)
    let rate_limiter = RateLimiter::new(100, Duration::from_secs(60));

    let feed_service = Arc::new(FeedService::new(db.clone(), config.clone()));
    let message_service = Arc::new(MessageService::new(db, config.clone()));

    // Configure logging
    let trace_layer = TraceLayer::new_for_http()
        .on_request(|request: &axum::http::Request<_>, _: &_| {
            tracing::info!(
                ">> Request {} {} {:?}",
                request.method(),
                request.uri(),
                request.headers()
            );
        })
        .on_response(
            |response: &axum::http::Response<_>, latency: Duration, _: &_| {
                tracing::info!(
                    "<< Response {} {:?} ({}ms)",
                    response.status(),
                    response.headers(),
                    latency.as_millis()
                );
            },
        );

    let app = Router::new()
        .route("/health", get(check))
        .route("/api/contact", axum::routing::post(handle_message))
        .layer(cors)
        .layer(trace_layer)
        .layer(rate_limiter)
        .with_state(message_service)
        .with_state(feed_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server running on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
