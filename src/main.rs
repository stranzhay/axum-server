mod utils;
use tracing::Level;

use axum::{routing::get, Router};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let port_env = env::var("PORT").expect("PORT must be set");
    let port = port_env.parse::<u16>().unwrap();

    // Declare API router and routes
    let app = Router::new().route("/", get(root));

    // Bind server to PORT and serve the router
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::event!(Level::INFO, "Axum start on {}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
