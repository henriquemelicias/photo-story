use std::backtrace::Backtrace;
use sqlx::{Executor, Pool, Transaction};
use thiserror::Error;

pub type DB = Pool<sqlx::Postgres>;

pub trait Queryer<'a>: Executor<'a, Database = sqlx::Postgres> {}

impl<'a> Queryer<'a> for &Pool<sqlx::Postgres> {}
// impl<'a> Queryer<'a> for &'a mut Transaction<'_, sqlx::Postgres> {}
