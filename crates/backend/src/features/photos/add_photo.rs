use chrono::DateTime;
use uuid::Uuid;

use crate::{
    domain::entities,
    features::photos::{self, Service},
};

#[derive(Debug, Clone)]
pub struct Input {
    pub url:         String,
    pub title:       String,
    pub description: Option<String>,
}

impl Service {
    pub async fn add_photo( &self, input: Input ) -> Result<entities::Photo, photos::Error> {
        let photo = entities::Photo {
            id:          Uuid::new_v4(),
            created_at:  DateTime::default(),
            url:         input.url,
            title:       input.title,
            description: input.description,
            updated_at:  DateTime::default(),
        };

        // TODO self.repo.create_photo(&self.db, &photo ).await?;

        Ok( photo )
    }
}
