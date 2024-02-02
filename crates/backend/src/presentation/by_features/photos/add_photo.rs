use axum::{
    body::Bytes,
    extract::{Host, OriginalUri},
    http,
    http::header,
    response::IntoResponse,
    Extension,
};

use super::Error;
use crate::features;

#[axum::debug_handler]
pub async fn add_photo(
    Host( host ): Host,
    OriginalUri( original_uri ): OriginalUri,
    Extension( photos_service ): Extension<features::photos::Service>,
    body: Bytes,
) -> Result<impl IntoResponse, Error> {

    unimplemented!();
    //let add_photo_input: common::api::photos::add_photo::Input =
    //    deserialize( &body ).map_err( |err| Error::Internal( err.to_string() ) )?;

    //let id = photos_service
    //    .add_photo( add_photo_input )
    //    .await
    //    .map_err( |err| match err {
    //        features::photos::error::internal( err ) => error::internal( err.to_string() ),
    //    } )?;

    //let location = [ host, original_uri, String::from("/"), id ].concat();
    //let headers = [( header::LOCATION, location )];
    // TODO: REMOVE THIS AFTER IMPLEMENTATION
    let headers = [(header::LOCATION, "REMOVE_THIS_AFTER_IMPLEMENTATION")];
    Ok( ( http::StatusCode::CREATED, headers ) )
}
