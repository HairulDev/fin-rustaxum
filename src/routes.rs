use axum::{
    routing::post,
    Router,
};
use std::sync::Arc;

use crate::{
    handlers::upload::upload_file,
    models::AppState,
};

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/upload", post(upload_file))
        .with_state(app_state)
}
