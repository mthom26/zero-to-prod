use std::net::SocketAddr;

use zero_to_prod::app;

#[tokio::test]
async fn health_check_works() {
    let server_addr = serve();
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
    let server_addr = serve();
    let addr = format!("http://{}/subscriptions", server_addr);

    let body = "name=jimmy%20derp&email=jimderp%40derpmail.com";
    let client = reqwest::Client::new();
    let response = client
        .post(addr)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Could not execute request.");

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_form_data() {
    let server_addr = serve();
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
            400,
            "The API did not fail when {}",
            error
        )
    }
}

fn serve() -> SocketAddr {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let app = app();

    let server = axum::Server::bind(&addr).serve(app.into_make_service());
    let local_addr = server.local_addr();
    tokio::spawn(async move { server.await });

    local_addr
}
