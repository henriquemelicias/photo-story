use chrono::{DateTime, Utc};

use crate::infrastructure::{
    drivers::db,
    repository::{self, Repository},
};

#[derive(Debug, Clone)]
pub struct Input {
    pub created_at:  DateTime<Utc>,
    pub url:         String,
    pub title:       String,
    pub description: Option<String>,
}

#[derive(sqlx::FromRow)]
struct Output {
    id: i32,
}

impl Repository {
    pub async fn create_photo<'a, T: db::Queryer<'a>>( &self, db: T, input: Input ) -> Result<u32, repository::Error> {
        let query = r#"
            INSERT INTO photos ( created_at, url, title, description )
            VALUES ( $1, $2, $3, $4 )
            RETURNING id;
        "#;

        let row = sqlx::query_as::<_, Output>( query )
            .bind( input.created_at )
            .bind( &input.url )
            .bind( &input.title )
            .bind( &input.description )
            .fetch_one( db )
            .await
            .map_err( |err| repository::Error::QueryFailed( "photos.create_photo", err ) )?;

        u32::try_from( row.id ).map_err( repository::Error::IntConversionFailed )
    }
}
