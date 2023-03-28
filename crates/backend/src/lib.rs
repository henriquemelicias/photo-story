#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

pub mod logger;
pub mod settings;

#[cfg( test )]
mod tests;

mod app;
mod domain;
mod features;
mod infrastructure;
mod services;
mod utils;

use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

#[tokio::main]
pub async fn init_server( addr: &str, port: u16 )
{
    let app = app::create().await;

    /* Serve server. */
    let sock_addr = SocketAddr::from( (
        IpAddr::from_str( addr ).unwrap_or( IpAddr::V6( Ipv6Addr::LOCALHOST ) ),
        port,
    ) );

    tracing::info!( "Listening on https://{}", sock_addr );

    axum::Server::bind( &sock_addr )
        .serve( app.into_make_service() )
        .await
        .expect( "Unable to start server" );
}
