use thiserror::Error;

pub mod photos;

#[derive(Debug, Clone)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Self { Self {} }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error( "Failed to execute database query in {0} due to: {1}." )]
    QueryFailed( &'static str, #[source] sqlx::Error ),
    #[error( "Failed to cast due to: {0}." )]
    IntConversionFailed( #[from] std::num::TryFromIntError ),
}
