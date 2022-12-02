use axum::{routing::get, Router};
use sync_wrapper::SyncWrapper;
mod handlers;
use handlers::*;
mod utils;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new()
        .route("/hello", get(hello_world))
        .route("/getNFT/mint/:id/network/:network", get(fetch_nft_handler));

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
