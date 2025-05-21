use axum::Router;
use dotenvy::dotenv;
use reqwest::{Client, Method, header};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();

    // create a shared reqwest client
    let http_client = Client::new();

    // Define CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION]);

    // Build application routes
    let app = Router::new()
        // .route("/api/chat", post(<post_method>))
        .with_state(http_client)
        .layer(cors);

    // create app listening address
    let addr = format!("0.0.0.0:{}", 3000); // can change 3000 to dynamic value

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
