mod utils;
use axum::http::{HeaderValue, Method};
use axum::routing::get;
use tower_http::cors::CorsLayer;
use tracing::Level;
mod handlers;
mod state;

use handlers::*;

use axum::Router;
use dotenv::dotenv;
use std::net::SocketAddr;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT").unwrap_or(String::from("8080"));

    // api routes and router
    let app = Router::new()
        .route("/getNFT/mint/:id/network/:network", get(fetch_nft_handler))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::POST, Method::GET]),
        )
        .layer(
            CorsLayer::new()
                // add deployed front end url here
                .allow_origin("".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::POST, Method::GET]),
        );

    // bind port and server then serve router
    let addr = SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap();
    tracing::event!(Level::INFO, "Axum start on {}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
