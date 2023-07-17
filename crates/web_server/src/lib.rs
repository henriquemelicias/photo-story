#![feature( unwrap_infallible )]
#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

pub use error::Error;

use axum::ServiceExt;
use error_stack::{report, IntoReport, Report, ResultExt};
use leptos::LeptosOptions;
use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    ops::Deref,
    str::FromStr,
};
use thiserror::Error;
use tracing::instrument;

pub mod logger;
pub mod settings;

mod app;
mod error;

#[cfg( feature = "ssr" )]
mod ssr;

#[derive(Error, Debug)]
pub enum InitServerError
{
    /// Failed to create the app.
    #[error( "Failed to create the app router." )]
    AppRouterCreationFailed,
    /// Failed to bind on the provided address.
    #[error( "Failed to bind on the address: {0}" )]
    AddressBindFailed( SocketAddr ),
    /// Failed to serve the server.
    #[error( "Failed to serve the server." )]
    ServerServeFailed,
}

/// Initialize the server.
///
/// # Arguments
///
/// * `server_settings` - The server settings [`ServerConfigs`].
///
#[tokio::main]
#[instrument( name = "APP", err, skip_all )]
pub async fn init_server(
    server_settings: settings::ServerConfigs,
    leptos_options: LeptosOptions,
) -> Result<(), Report<InitServerError>>
{
    tracing::info!(
        "Initializing server the with settings: [ sock_addr={}, proxy_url={}, static_dir={}, assets_dir={} ].",
        server_settings.sock_addr_v4,
        server_settings.proxy_url.as_str(),
        server_settings.static_dir,
        server_settings.assets_dir
    );

    let app = app::create(
        server_settings.static_dir,
        server_settings.assets_dir,
        server_settings.proxy_url,
        leptos_options,
    )
    .await
    .ok_or_else( || report!( InitServerError::AppRouterCreationFailed ) )?;

    let server = axum::Server::try_bind( &server_settings.sock_addr_v4.into() )
        .into_report()
        .change_context( InitServerError::AddressBindFailed( server_settings.sock_addr_v4.into() ) )?;

    tracing::info!( "Server bound to http://{} successfully.", server_settings.sock_addr_v4 );
    tracing::info!( "Serving files..." );

    server
        .serve( app.into_make_service() )
        .await
        .into_report()
        .change_context( InitServerError::ServerServeFailed )?;

    Ok( () )
}
