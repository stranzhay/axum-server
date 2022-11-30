mod utils;
use axum::http::{HeaderValue, Method};
use axum::routing::post;
use tower_http::cors::CorsLayer;
use tracing::Level;
mod handlers;
mod state;

use handlers::*;

use axum::Router;
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let port_env = env::var("PORT").expect("PORT must be set");
    let port = port_env.parse::<u16>().unwrap();

    // api routes and router
    let app = Router::new()
        .route("/getNFT", post(fetch_nft_handler))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::POST]),
        )
        .layer(
            CorsLayer::new()
                // add deployed front end url here
                .allow_origin("".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::POST]),
        );

    // bind port and server then serve router
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::event!(Level::INFO, "Axum start on {}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
