use std::net::SocketAddr;

use axum::{
    Router,
    routing::{get, post},
};

// import dotenv function
use dotenvy::dotenv;
use handler::{chat_request, root_handler};
use reqwest::{Method, header};
use tower_http::cors::{Any, CorsLayer};

mod handler;
mod model;
mod openai_api;

#[tokio::main]
async fn main() {
    // load the environment files
    // ok is added to ignore if there are any error in fetching env file
    dotenv().ok();

    // add cors layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(vec![header::CONTENT_TYPE])
        .allow_methods(vec![Method::GET, Method::POST]);

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/api/chat", post(chat_request))
        .layer(cors);

    let addr_str = "0.0.0.0:3000";

    let addr: SocketAddr = addr_str
        .parse()
        .expect("Listnening to provided Port is failed");

    println!("App is listening at http:/{}", addr_str);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
