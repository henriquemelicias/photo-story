use thiserror::Error;

mod photos;

#[derive(Debug)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Self { Self {} }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error( "Failed to execute database query in {method} due to: {source}." )]
    QueryFailed {
        method: String,
        #[source]
        source: sqlx::Error,
    },
}
