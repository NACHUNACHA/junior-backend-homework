use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct File {
    pub id: i32,
    pub url: String,
    pub filename: String,
    pub original_name: String,
    pub mime_type: Option<String>,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

