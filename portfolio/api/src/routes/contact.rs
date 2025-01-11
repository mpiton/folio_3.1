use crate::{models::contact::ContactRequest, AppState};
use axum::{
    extract::{ConnectInfo, Json, State},
    response::IntoResponse,
};
use std::net::SocketAddr;

pub async fn submit_contact(
    State(state): State<AppState>,
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    Json(request): Json<ContactRequest>,
) -> impl IntoResponse {
    match state.contact_service.submit_contact(request.form).await {
        Ok(_) => "Message envoyé avec succès".into_response(),
        Err(e) => format!("Erreur lors de l'envoi du message: {}", e).into_response(),
    }
}
