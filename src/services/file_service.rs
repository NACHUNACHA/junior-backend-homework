use sqlx::{MySqlPool, Row};
use crate::repositories::file_repository;
use uuid::Uuid;
use std::error::Error;
use rocket::http::Status;
use rocket_multipart_form_data::FileField;
use crate::dtos::file_dto::FileResponse;
use crate::s3_client;

pub async fn get_all_files(pool: &MySqlPool) -> Result<Vec<FileResponse>, sqlx::Error> {
    file_repository::get_all(pool).await
}

pub async fn upload_file(
    pool: &MySqlPool,
    file: &FileField,
) -> Result<Status, Box<dyn Error>> {

    let original_name = file.file_name.clone().unwrap().to_string();
    let mime_type = file.content_type.clone().unwrap().to_string();
    let size = file.path.metadata()?.len();
    let file_ext = original_name.split('.').last().unwrap_or("").to_string();
    let unique_filename = format!("{}.{}", Uuid::new_v4(), file_ext);
    
    let s3_client = s3_client::get_client().await?;
    let bucket_name = std::env::var("AWS_BUCKET_NAME")?;
    
    let temp_path = file.path.to_path_buf();

    println!("{:?}", temp_path);
    
    s3_client::upload(&s3_client, &bucket_name, &unique_filename, &temp_path, &mime_type).await?;

    let url = format!("https://{}.s3.amazonaws.com/{}", bucket_name, unique_filename);
    println!("{}", url);
    
    file_repository::create(
        pool,
        &url,
        &unique_filename,
        &original_name,
        &mime_type,
        size
    ).await?;

    Ok(Status::Created)
}