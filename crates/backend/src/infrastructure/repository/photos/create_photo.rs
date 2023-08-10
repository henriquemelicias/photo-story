use sqlx::postgres::PgQueryResult;

use crate::{
    domain::entities,
    infrastructure::{drivers::db, repository, repository::Repository},
};

impl Repository {
    pub async fn create_photo<'a, T: db::Queryer<'a>>(
        &self,
        db: T,
        photo: &entities::Photo,
    ) -> Result<PgQueryResult, repository::Error> {
        let query = r#"
            INSERT INTO photos ( id, created_at, updated_at, title, description, url )
            VALUES ( $1, $2, $3, $4, $5, $6 )
        "#;

        let result = sqlx::query( query )
            .bind( photo.id )
            .bind( photo.created_at )
            .bind( photo.updated_at )
            .bind( &photo.title )
            .bind( &photo.description )
            .bind( &photo.url )
            .execute( db )
            .await;

        result.map_err( |err| repository::Error::QueryFailed {
            method: "photos.create_photo".to_string(),
            source: err,
        } )
    }
}
