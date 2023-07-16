use crate::infrastructure::drivers::db::DB;
use crate::infrastructure::repository::Repository;

pub mod photos;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB
}

impl Service
{
    pub fn new(db: DB) -> Self
    {
        Self
        {
            repo: Repository::new(),
            db
        }
    }
}