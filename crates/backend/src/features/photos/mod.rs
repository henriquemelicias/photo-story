mod add_photo;

use thiserror::Error;

use crate::infrastructure::{drivers::db, repository, repository::Repository};

#[derive(Error, Debug)]
pub enum Error {
    // Photos.

    // Other.
    #[error( transparent )]
    Internal( #[from] repository::Error ),
}

#[derive(Debug, Clone)]
pub struct Service {
    db:   db::Pool,
    repo: Repository,
}

impl Service {
    pub fn new( db: db::Pool ) -> Self {
        Self {
            db,
            repo: Repository::new(),
        }
    }
}
