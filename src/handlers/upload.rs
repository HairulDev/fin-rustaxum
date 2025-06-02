use axum::{
    extract::{Multipart, State},
    response::Json,
};
use std::{path::Path, sync::Arc};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::models::{AppState, UploadResponse};

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, axum::http::StatusCode> {
    let mut saved_files = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        eprintln!("Failed to read multipart field: {:?}", e);
        axum::http::StatusCode::BAD_REQUEST
    })? {
        let file_name = field.file_name().unwrap_or("file").to_string();
        let file_id = Uuid::new_v4();

        let extension = Path::new(&file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let final_name = if extension.is_empty() {
            file_id.to_string()
        } else {
            format!("{}.{}", file_id, extension)
        };
        eprintln!("Reading field: {:?}", field.file_name());

        let final_path = format!("{}/{}", state.upload_dir, final_name);
        let mut file = File::create(&final_path).await.map_err(|e| {
            eprintln!("Failed to create file: {:?}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let mut field_stream = field;

        while let Some(chunk) = field_stream.chunk().await.map_err(|e| {
            eprintln!("Failed to read chunk: {:?}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })? {
            file.write_all(&chunk).await.map_err(|e| {
                eprintln!("Failed to write chunk: {:?}", e);
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            })?;
        }

        saved_files.push(final_name);
    }

    Ok(Json(UploadResponse { files: saved_files }))
}
