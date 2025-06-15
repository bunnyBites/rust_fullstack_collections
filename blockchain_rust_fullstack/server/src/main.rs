use std::{str::FromStr, sync::Arc};

use anchor_client::{
    Client, Cluster,
    solana_sdk::{pubkey::Pubkey, signature::Keypair},
};
use axum::{
    Router,
    http::{Method, header},
    routing::get,
};
use tower_http::cors::{Any, CorsLayer};

use crate::handler::get_todos;

mod handler;
mod model;

const PROGRAM_ID: &str = "AUEPaukkyUiQVu5YFb76mJU9yfzXzi78qrKQnbK8H3c1";

#[tokio::main]
async fn main() {
    // create Rpc client
    let rpc_url = Cluster::Localnet;
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();

    let dummy_payer = Keypair::new();
    let client = Client::new(rpc_url, Arc::new(dummy_payer));

    let program = client.program(program_id).unwrap();
    let program_state = Arc::new(program);

    // create cors layer
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(vec![header::CONTENT_TYPE]);

    // create routes with rpc clien
    let app = Router::new()
        .route("/sol/{user_pubkey}", get(get_todos))
        .layer(cors_layer)
        .with_state(program_state);

    // prepare listener
    let listener_port = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(listener_port).await.unwrap();

    println!("Server is running on http:/{}", listener_port);

    // serve the app
    axum::serve(listener, app).await.unwrap();
}
