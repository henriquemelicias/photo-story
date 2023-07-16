#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

#![feature(error_generic_member_access)]
#![feature(provide_any)]

use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};
use std::net::SocketAddrV4;
use error_stack::{Report, ResultExt, report, IntoReport};

use tracing::instrument;
pub use error::Error;

use thiserror::Error;
mod domain;
mod infrastructure;
mod presentation;
mod features;
mod utils;
mod error;
pub mod logger;
pub mod settings;


#[derive(Error, Debug)]
pub enum InitServerError
{
    /// Failed to create the app.
    #[error("Failed to create the app router.")]
    AppRouterCreationFailed,
    /// The address provided is invalid.
    #[error("The address provided ({0}) is invalid.")]
    InvalidAddr(&'static str),
    /// Failed to bind on the provided address.
    #[error("Failed to bind on the address: {0}")]
    AddressBindFailed( SocketAddrV4 ),
    /// Failed to serve the server.
    #[error("Failed to serve the server.")]
    ServerServeFailed,
}

#[tokio::main]
#[instrument(name = "APP", err, skip(server_settings))]
pub async fn init_server( server_settings: settings::ServerConfigs ) -> Result<(), Report<InitServerError>>
{
    tracing::info!( "Initializing server the with settings: [ sock_addr_v4={}, frontend_url={} ].", server_settings.sock_addr_v4, server_settings.frontend_url.as_str() );

    let app = presentation::app::create( server_settings.frontend_url ).await.ok_or_else(|| report!( InitServerError::AppRouterCreationFailed ) )?;

    let server = axum::Server::try_bind( &server_settings.sock_addr_v4.into() )
        .into_report()
        .change_context( InitServerError::AddressBindFailed( server_settings.sock_addr_v4 ) )?;

    tracing::info!( "Server bound to http://{} successfully.", server_settings.sock_addr_v4 );

    server.serve( app.into_make_service() )
        .await
        .into_report()
        .change_context( InitServerError::ServerServeFailed )?;

    Ok( () )
}
