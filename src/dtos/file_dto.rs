use serde::Serialize;

#[derive(Serialize)]
pub struct FileResponse {
    pub id: i32,
    pub url: String,
    pub filename: String,
    pub original_name: String,
    pub mime_type: String,
    pub size: u64,
}
