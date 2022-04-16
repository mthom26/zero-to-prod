use std::net::SocketAddr;

use zero_to_prod::app;

#[tokio::test]
async fn health_check_works() {
    serve();

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health-check")
        .send()
        .await
        .expect("Could not execute request.");

    assert_eq!(response.status(), 200);
    assert_eq!(response.content_length(), Some(0));
}

fn serve() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let app = app();

    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    tokio::spawn(async move { server.await });
}
