use chrono::DateTime;

use crate::{
    features::photos::{self, Service},
    infrastructure::repository,
};

impl Service {
    pub async fn add_photo( &self, input: common::api::photos::add_photo::Input ) -> Result<u32, photos::Error> {
        let repo_input = repository::photos::create_photo::Input {
            created_at:  DateTime::default(),
            url:         input.url,
            title:       input.title,
            description: input.description,
        };

        self.repo
            .create_photo( &self.db, repo_input )
            .await
            .map_err( photos::Error::Internal )
    }
}
