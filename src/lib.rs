use axum::{
    extract::Form,
    http::StatusCode,
    routing::{get, post},
    Router,
};

pub fn app() -> Router {
    Router::new()
        .route("/health-check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe(form_data: Form<FormData>) -> StatusCode {
    StatusCode::OK
}

#[derive(Debug, serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}
