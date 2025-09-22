use sqlx::{Error, MySqlPool, Result, query_as};
use sqlx::mysql::MySqlRow;
use crate::dtos::file_dto::FileResponse;

pub async fn get_all(pool: &MySqlPool) -> Result<Vec<FileResponse>> {
    query_as!(
        FileResponse,
        "SELECT id, url, filename, original_name, mime_type, size FROM files"
    )
    .fetch_all(pool)
    .await
}

pub async fn create(
    pool: &MySqlPool,
    url: &str,
    filename: &str,
    original_name: &str,
    mime_type: &str,
    size: u64,
) -> std::result::Result<MySqlRow, Error> {
    query_as!(
        File,
        "INSERT INTO files (url, filename, original_name, mime_type, size) VALUES (?, ?, ?, ?, ?)",
        url,
        filename,
        original_name,
        mime_type,
        size
    )
        .fetch_one(pool)
        .await
}