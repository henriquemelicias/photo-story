use sqlx::{Executor, Postgres};
use std::time::Duration;
use thiserror::Error;

pub type Pool = sqlx::Pool<Postgres>;
type PoolOptions = sqlx::pool::PoolOptions<Postgres>;

pub trait Queryer<'a>: Executor<'a, Database = Postgres> {}
impl<'a> Queryer<'a> for &Pool {}

#[allow( unused )]
pub type Tx = sqlx::Transaction<'static, Postgres>;

#[derive(Error, Debug)]
#[error( "Sqlx pool connection failed due to: {0}." )]
pub struct ConnectionError( #[from] sqlx::Error );

pub async fn connect( connection: &str, max_connections: u32, max_lifetime_minutes: u32 ) -> Result<Pool, ConnectionError> {
    let pool = PoolOptions::new()
        .max_connections( max_connections )
        .max_lifetime( Duration::from_secs( u64::from( max_lifetime_minutes ) * 60 ) )
        .connect( connection )
        .await?;

    Ok( pool )
}

#[derive(Error, Debug)]
#[error( "Sqlx migration failed due to: {0}" )]
pub struct MigrationError( #[from] sqlx::migrate::MigrateError );

pub async fn migrate( db: &Pool ) -> Result<(), MigrationError> {
    sqlx::migrate!( "./migrations/" ).run( db ).await?;
    Ok( () )
}
