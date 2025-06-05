use std::sync::Arc;

use axum::{Router, routing::get};
use handler::get_todos;
use solana_client::rpc_client::RpcClient;

mod handler;
mod model;

const SOLANA_BASE_URL: &str = "http://127.0.0.1:8899";

#[tokio::main]
async fn main() {
    // prepare client
    let rpc_client = Arc::new(RpcClient::new(SOLANA_BASE_URL));

    // prepare listening port
    let listener_port = "0.0.0.0:3000".to_string();

    // prepare app with routes and states
    let app = Router::new()
        .route("/sol/:user_public_key", get(get_todos))
        .with_state(rpc_client);

    // connect with TCP and prepare listner
    let listener = tokio::net::TcpListener::bind(&listener_port).await.unwrap();

    println!("Listening on: http:/{}", listener_port);

    // serve the app on the listner
    axum::serve(listener, app).await.unwrap();
}
