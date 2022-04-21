use std::{net::SocketAddr, time::Duration};

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use zero_to_prod::{config::get_config, startup::app};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = get_config().expect("Failed to read configuration.");
    let addr = SocketAddr::from(([127, 0, 0, 1], config.application_port));

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .connect(&config.database.connection_string())
        .await
        .expect("Could not connect to database.");

    let app = app(pool);

    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
