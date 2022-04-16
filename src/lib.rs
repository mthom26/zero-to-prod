use axum::{http::StatusCode, routing::get, Router};

pub fn app() -> Router {
    Router::new().route("/health-check", get(health_check))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
