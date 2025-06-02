mod handlers;
mod models;
mod routes;

use std::{fs, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use models::AppState;

#[tokio::main]
async fn main() {
    let upload_dir = "uploads".to_string();
    fs::create_dir_all(&upload_dir).unwrap();

    let app_state = Arc::new(AppState { upload_dir });

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    let app = routes::create_routes(app_state)
        .layer(cors);

    println!("🚀 Server running on http://localhost:8080");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
