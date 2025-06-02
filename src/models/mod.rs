use serde::Serialize;

#[derive(Serialize)]
pub struct UploadResponse {
    pub files: Vec<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub upload_dir: String,
}
