use crate::domain::entities;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod add_photo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo
{
    pub id:          Uuid,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>,
    pub url:         String,
    pub title:       String,
    pub description: Option<String>,
}

impl From<entities::Photo> for Photo
{
    fn from( photo: entities::Photo ) -> Self
    {
        Self {
            id:          photo.id,
            created_at:  photo.created_at,
            updated_at:  photo.updated_at,
            url:         photo.url,
            title:       photo.title,
            description: photo.description,
        }
    }
}
