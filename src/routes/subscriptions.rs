use axum::{extract::Form, http::StatusCode};

pub async fn subscribe(form_data: Form<FormData>) -> StatusCode {
    StatusCode::OK
}

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
