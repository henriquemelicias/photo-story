use std::backtrace::Backtrace;
use thiserror::Error;

mod photos;

#[derive(Debug)]
pub struct Repository {}

impl Repository
{
    pub fn new() -> Self { Self {} }
}

#[derive(Error, Debug)]
pub enum Error
{
    #[error( "Failed to execute database query in {method:?}" )]
    QueryFailed
    {
        method:    String,
        #[source]
        source:    sqlx::Error,
        #[backtrace]
        backtrace: Backtrace,
    },
}
