use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::limit::RequestBodyLimitLayer;

use crate::{handlers::download::download_file, handlers::upload::upload_file, models::AppState};

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/upload", post(upload_file))
        .route("/download/:file_name", get(download_file))
        .layer(RequestBodyLimitLayer::new(50 * 1024 * 1024)) // set limit 50MB
        .with_state(app_state)
}
