use std::net::SocketAddr;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root_handler));

    let addr_str = "0.0.0.0:3000";

    let addr: SocketAddr = addr_str
        .parse()
        .expect("Listnening to provided Port is failed");

    println!("App is listening at http:/{}", addr_str);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Hello World !!"
}
