use std::{net::SocketAddr, time::Duration};

use sqlx::{postgres::PgPoolOptions, Connection, Executor, PgConnection};
use uuid::Uuid;

use zero_to_prod::{
    config::{get_config, Settings},
    startup::app,
};

#[tokio::test]
async fn health_check_works() {
    let (server_addr, _config) = spawn_app().await;
    let addr = format!("http://{}/health-check", server_addr);

    let client = reqwest::Client::new();
    let response = client
        .get(addr)
        .send()
        .await
        .expect("Could not execute request.");

    assert_eq!(response.status(), 200);
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let (server_addr, config) = spawn_app().await;
    let addr = format!("http://{}/subscriptions", server_addr);
    let conn_string = config.database.connection_string();

    let mut conn = PgConnection::connect(&conn_string)
        .await
        .expect("Could not connect to Postgres.");

    let body = "name=jimmy%20derp&email=jimderp%40derpmail.com";
    let client = reqwest::Client::new();
    let response = client
        .post(addr)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Could not execute request.");

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut conn)
        .await
        .expect("Could not fetch saved subscription.");

    assert_eq!(response.status(), 200);
    assert_eq!(saved.email, "jimderp@derpmail.com");
    assert_eq!(saved.name, "jimmy derp");
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_form_data() {
    let (server_addr, _config) = spawn_app().await;
    let addr = format!("http://{}/subscriptions", server_addr);

    let test_bodies = [
        ("name=jimmy%20derp", "missing email"),
        ("email=jimderp%40derpmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (body, error) in test_bodies {
        let client = reqwest::Client::new();
        let response = client
            .post(addr.clone())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Could not execute request.");

        assert_eq!(
            response.status(),
            422, // Axum responds with a 422 here
            "The API did not fail when {}",
            error
        )
    }
}

async fn spawn_app() -> (SocketAddr, Settings) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));

    let db_name = Uuid::new_v4().to_string();

    let mut config = get_config().expect("Failed to read configuration.");
    config.database.database_name = db_name;

    let mut conn = PgConnection::connect(&config.database.connection_string_no_db())
        .await
        .expect("Could not connect to Postgres.");

    let query = format!("CREATE DATABASE \"{}\";", config.database.database_name);
    conn.execute(query.as_str())
        .await
        .expect("Could not create database.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .connect(&config.database.connection_string())
        .await
        .expect("Could not connect to database.");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Could not migrate database.");

    let app = app(pool);

    let server = axum::Server::bind(&addr).serve(app.into_make_service());
    let local_addr = server.local_addr();
    tokio::spawn(async move { server.await });

    (local_addr, config)
}
