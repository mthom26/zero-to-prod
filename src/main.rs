use std::net::SocketAddr;

use zero_to_prod::{config::get_config, startup::app};

#[tokio::main]
async fn main() {
    let config = get_config().expect("Failed to read configuration.");
    let addr = SocketAddr::from(([127, 0, 0, 1], config.application_port));
    let app = app();

    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
