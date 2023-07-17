use chrono::{DateTime, Utc};
use std::path::Path;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Photo
{
    pub id:         Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub url:         String,
    pub title:       String,
    pub description: Option<String>,
}
