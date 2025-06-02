use crate::models::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use mime_guess::from_path;
use std::{fs::File, io::Read, path::PathBuf, sync::Arc};

pub async fn download_file(
    Path(file_name): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let file_path = PathBuf::from(format!("{}/{}", state.upload_dir, file_name));

    if !file_path.exists() {
        return (StatusCode::NOT_FOUND, "File not found".to_string()).into_response();
    }

    // Baca file ke dalam buffer
    let mut file = File::open(&file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // Tentukan Content-Type berdasarkan ekstensi
    let mime_type = from_path(&file_path).first_or_octet_stream().to_string();

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&mime_type).unwrap(),
    );

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_str(&mime_type).unwrap(),
        )
        .body(buffer.into())
        .unwrap()
}
