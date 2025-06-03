use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;
use std::{fs, path::PathBuf, sync::Arc};

use crate::models::AppState;

#[derive(Serialize)]
struct DeleteResponse {
    message: String,
}

pub async fn delete_file(
    State(state): State<Arc<AppState>>,
    Path(file_name): Path<String>,
) -> impl IntoResponse {
    let file_path: PathBuf = PathBuf::from(format!("{}/{}", state.upload_dir, file_name));

    if file_path.exists() {
        match fs::remove_file(&file_path) {
            Ok(_) => {
                let res = DeleteResponse {
                    message: "File deleted successfully".to_string(),
                };
                (StatusCode::OK, Json(res)).into_response()
            }
            Err(e) => {
                eprintln!("Failed to delete file: {:?}", e);
                let res = DeleteResponse {
                    message: "Failed to delete file".to_string(),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(res)).into_response()
            }
        }
    } else {
        let res = DeleteResponse {
            message: "File not found".to_string(),
        };
        (StatusCode::NOT_FOUND, Json(res)).into_response()
    }
}
