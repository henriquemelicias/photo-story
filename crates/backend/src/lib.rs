#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};
use anyhow::Context;

pub mod logger;
pub mod settings;

mod app;
mod domain;
mod features;
mod infrastructure;
mod services;
mod utils;

#[tokio::main]
pub async fn init_server( addr: &str, port: u16 ) -> anyhow::Result<()>
{
    let app = app::create().await.context( "Unable to create app" )?;

    /* Serve server. */
    let sock_addr = SocketAddr::from( (
        IpAddr::from_str( addr ).unwrap_or( IpAddr::V6( Ipv6Addr::LOCALHOST ) ),
        port,
    ) );

    tracing::info!( "Listening on https://{}", sock_addr );

    axum::Server::bind( &sock_addr )
        .serve( app.into_make_service() )
        .await
        .context( "Unable to start server" )?;

    Ok( () )
}
