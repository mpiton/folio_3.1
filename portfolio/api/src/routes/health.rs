use axum::response::IntoResponse;

pub async fn check() -> impl IntoResponse {
    "Service OK".into_response()
}
