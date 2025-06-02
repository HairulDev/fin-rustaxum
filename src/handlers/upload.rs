use axum::{
    extract::{Multipart, State},
    response::Json,
};
use std::{fs, path::Path, sync::Arc};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::models::{AppState, UploadResponse};

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Json<UploadResponse> {
    let mut saved_files = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("file").to_string();
        let file_id = Uuid::new_v4();

        let temp_path = format!("{}/{}", state.upload_dir, file_id);
        let mut file = File::create(&temp_path).await.unwrap();
        let data = field.bytes().await.unwrap();
        file.write_all(&data).await.unwrap();

        let extension = Path::new(&file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let final_name = if extension.is_empty() {
            file_id.to_string()
        } else {
            format!("{}.{}", file_id, extension)
        };

        let final_path = format!("{}/{}", state.upload_dir, final_name);
        fs::rename(&temp_path, &final_path).unwrap();
        saved_files.push(final_path.clone());

        println!(
            "Uploaded file: {} ({} bytes) =====>>>",
            final_name,
            data.len()
        );
    }

    Json(UploadResponse { files: saved_files })
}
