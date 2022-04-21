use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPool;

use crate::routes::{health_check, subscribe};

pub fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/health-check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(Extension(pool))
}
