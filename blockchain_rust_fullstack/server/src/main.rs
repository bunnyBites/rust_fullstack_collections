use std::sync::Arc;

use axum::{Router, routing::get};
use solana_client::rpc_client::RpcClient;

use crate::handler::get_todos;

mod handler;
mod model;

const SOLANA_BASE_URL: &str = "http://127.0.0.1:8899";

#[tokio::main]
async fn main() {
    // create Rpc client
    let rpc_client = Arc::new(RpcClient::new(SOLANA_BASE_URL));

    // create routes with rpc clien
    let app = Router::new()
        .route("/sol/{user_pubkey}", get(get_todos))
        .with_state(rpc_client);

    // prepare listener
    let listener_port = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(listener_port).await.unwrap();

    println!("Server is running on http:/{}", listener_port);

    // serve the app
    axum::serve(listener, app).await.unwrap();
}
