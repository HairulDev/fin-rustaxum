use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{delete, get, post},
};
use std::sync::Arc;
use tower_http::limit::RequestBodyLimitLayer;

use crate::{
    handlers::{delete::delete_file, download::download_file, upload::upload_file},
    models::AppState,
};

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/upload", post(upload_file))
        .route("/download/:file_name", get(download_file))
        .route("/delete/:file", delete(delete_file))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(200 * 1_000_000)) // 200 MB limit
        .with_state(app_state)
}
