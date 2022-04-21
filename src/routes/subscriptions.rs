use axum::{extract::Form, http::StatusCode, Extension};
use chrono::Utc;
use sqlx::postgres::PgPool;
use uuid::Uuid;

pub async fn subscribe(
    form_data: Form<FormData>,
    Extension(pool): Extension<PgPool>,
) -> StatusCode {
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form_data.email,
        form_data.name,
        Utc::now()
    )
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
