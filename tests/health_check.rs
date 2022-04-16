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

fn serve() -> SocketAddr {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let app = app();

    let server = axum::Server::bind(&addr).serve(app.into_make_service());
    let local_addr = server.local_addr();
    tokio::spawn(async move { server.await });

    local_addr
}
