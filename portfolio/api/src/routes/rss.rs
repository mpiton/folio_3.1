use crate::AppState;
use axum::{
    extract::State,
    response::{IntoResponse, Json},
};

pub async fn get_rss_items(State(state): State<AppState>) -> impl IntoResponse {
    let url = "https://example.com/feed.xml"; // À remplacer par l'URL réelle
    match state.rss_service.fetch_and_store_feed(url).await {
        Ok(items) => Json(items).into_response(),
        Err(e) => {
            tracing::error!("Erreur lors de la récupération des éléments RSS: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Erreur lors de la récupération des éléments RSS",
            )
                .into_response()
        }
    }
}
