use axum::{http::StatusCode, response::IntoResponse};

pub async fn subscriptions() -> impl IntoResponse {
    (StatusCode::OK, "subscriptions endpoint")
}
