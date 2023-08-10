mod add_photo;

use crate::infrastructure::{drivers::db, repository::Repository};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {}

#[derive(Debug)]
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
