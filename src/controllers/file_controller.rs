use std::io::Error;
use rocket::{post, State, Data, Response};
use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;
use rocket_multipart_form_data::{mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions};
use sqlx::MySqlPool;
use crate::dtos::file_dto::FileResponse;
use crate::services::file_service;

#[get("/files")]
pub async fn list_files(pool: &State<MySqlPool>) -> Result<Json<Vec<FileResponse>>, Status> {
    match file_service::get_all_files(pool.inner()).await {
        Ok(files) => Ok(Json(files)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/files", data = "<data>")]
pub async fn upload_file(
    pool: &State<MySqlPool>,
    content_type: &ContentType,
    data: Data<'_>,
) -> Result<Status, Status> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec![
            MultipartFormDataField::file("file")
                .size_limit(5 * 1024 * 1024)
                .content_type_by_string(Some(mime::STAR_STAR))
                .unwrap()
        ]
    );

    let multipart = MultipartFormData::parse(content_type, data, options)
        .await
        .map_err(|_| Status::BadRequest)?;

    let file = multipart.files.get("file").unwrap();
    let file_field = file.first().ok_or(Status::BadRequest)?;

    let content_type = file_field.content_type
        .as_ref()
        .ok_or(Status::UnsupportedMediaType)?;

    let is_valid_type = match content_type.essence_str() {
        "application/pdf" => true,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => true,
        "image/jpeg" => true,
        "image/png" => true,
        _ => false
    };

    if !is_valid_type {
        return Err(Status::UnsupportedMediaType);
    }

    file_service::upload_file(pool, file_field)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Status::Created)
}