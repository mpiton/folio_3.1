use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

use crate::models::contact::Request;
use crate::services::contact::MessageService;

pub async fn handle_message(
    State(state): State<Arc<MessageService>>,
    Json(form): Json<Request>,
) -> impl IntoResponse {
    match state.submit_contact(form).await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Message envoyé avec succès"
            })),
        )
            .into_response(),
        Err(e) => {
            let error_msg = e.to_string();
            let status = if error_msg.contains("Validation error") {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };

            (
                status,
                Json(json!({
                    "status": "error",
                    "message": error_msg
                })),
            )
                .into_response()
        }
    }
}
