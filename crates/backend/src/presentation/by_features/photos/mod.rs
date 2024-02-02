use axum::{
    body::{Body, Bytes, Full},
    http,
    http::header,
    response::{IntoResponse, Response},
};
use thiserror::Error;

pub mod add_photo;

#[derive(Error, Debug)]
pub enum Error {
    #[error( "Unknown internal error." )]
    InternalUnknown,

    #[error( "Internal error due to: {0}" )]
    Internal( String ),
}

impl IntoResponse for Error {
    fn into_response( self ) -> Response {
        unimplemented!();
        //let ( status, error_message ) = match self {
        //    Error::InternalUnknown => ( http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string() ),
        //    Error::Internal( _ ) => ( http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string() ),
        //};

        //let response_body: Vec<u8> = serialize( &common::api::ErrorResponseBody { message: error_message } )
        //    .expect( "Failed to deserialize error struct into bytes. This should never fail." );

        //( status, response_body ).into_response()
    }
}
