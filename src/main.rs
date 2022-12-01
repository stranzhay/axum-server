mod utils;
use axum::response::Html;
use axum::routing::get;
use tracing::Level;
mod handlers;

use handlers::*;

use axum::Router;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT").unwrap_or(String::from("8080"));

    // api routes and router
    let app = Router::new()
        .route("/", get(handler))
        .route("/getNFT/mint/:id/network/:network", get(fetch_nft_handler));

    // bind port and server then serve router
    let addr = "[::]:8080".parse().unwrap();
    tracing::event!(Level::INFO, "Axum start on {}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello world</h1>")
}
