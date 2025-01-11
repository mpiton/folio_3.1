use portfolio_api::{
    config::Config,
    models::contact::ContactForm,
    routes::{health::health_check, rss::get_rss_items},
    services::{contact::ContactService, db::init_db, rss::RssService},
    AppState,
};

use axum::{
    extract::State,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode,
    },
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialiser le logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Charger la configuration
    let config = Config::new();
    let db = init_db().await.expect("Failed to initialize database");

    // Initialiser les services
    let app_state = AppState {
        rss_service: Arc::new(RssService::new(db.clone(), config.clone())),
        contact_service: Arc::new(ContactService::new(db, config.clone())),
    };

    // Configuration CORS
    let host = std::env::var("HOST").unwrap();
    let port = std::env::var("PORT").unwrap();
    let cors = CorsLayer::new()
        .allow_origin([
            format!("http://{}:{}", host, port).parse().unwrap(),
            format!("https://{}:{}", host, port).parse().unwrap(),
        ])
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION]);

    // Créer le routeur
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/rss", get(get_rss_items))
        .route("/contact", post(submit_contact))
        .layer(cors)
        .with_state(app_state);

    // Démarrer le serveur
    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn submit_contact(
    State(state): State<AppState>,
    Json(form): Json<ContactForm>,
) -> StatusCode {
    match state.contact_service.submit_contact(form).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let config = Config::new();
        let db = init_db().await.expect("Failed to initialize test database");

        let app_state = AppState {
            rss_service: Arc::new(RssService::new(db.clone(), config.clone())),
            contact_service: Arc::new(ContactService::new(db, config)),
        };

        let app = Router::new()
            .route("/health", get(health_check))
            .with_state(app_state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
