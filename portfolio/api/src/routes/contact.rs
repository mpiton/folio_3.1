use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;

use crate::models::contact::Request;
use crate::services::contact::MessageService;

pub async fn handle_message(
    State(state): State<Arc<MessageService>>,
    Json(form): Json<Request>,
) -> impl IntoResponse {
    match state.submit_contact(form).await {
        Ok(()) => "Message envoyé avec succès".into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {e}"),
        )
            .into_response(),
    }
}
