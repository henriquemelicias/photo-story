use std::path::Path;
use uuid::Uuid;
use crate::domain::entities;
use crate::features::Service;
use crate::features::photos;

#[derive(Debug, Clone)]
pub struct AddPhotoInput
{
    pub url: String,
    pub title: String,
    pub description: Option<String>,
}

impl Service
{
    pub async fn add_photo(&self, input: AddPhotoInput) -> Result<entities::Photo, photos::Error>
    {
        let photo = entities::Photo {
            id: Uuid::new_v4(),
            created_at: Default::default(),
            url: input.url,
            title: input.title,
            description: input.description,
            updated_at: Default::default(),
        };

        // TODO self.repo.create_photo(&self.db, &photo ).await?;

        Ok(photo)
    }
}