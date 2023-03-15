#![feature( once_cell )]
#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

// Modules.
pub mod services;
pub mod settings;

// Crate use re-exports.
pub use color_eyre::eyre::Result;

use monitoring::logger;

use axum::{
    http,
    Router,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    extract::State
};
use std::{
    convert::Infallible,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};
use dioxus::prelude::*;

use futures::stream::{self, StreamExt};
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
};

pub fn start_logs( log_level: &str ) -> ( Option<logger::WorkerGuard>, Option<logger::WorkerGuard> )
{
    let mut log_output_types = Vec::new();

    if *settings::LOGGER.is_stdout_emitted()
    {
        log_output_types.push( logger::OutputType::Stdout );
    }

    if *settings::LOGGER.is_file_emitted()
    {
        log_output_types.push( logger::OutputType::File {
            app_name:  settings::GENERAL.app_name(),
            directory: settings::LOGGER
                .files_directory()
                .as_ref()
                .expect( "Failed to get logger files directory" ),
            prefix:    settings::LOGGER
                .files_prefix()
                .as_ref()
                .expect( "Failed to get logger files prefix" ),
        } )
    }

    logger::init(
        &logger::Level::from_str( log_level ).expect( "Failed to parse log level" ),
        &log_output_types,
    )
}

#[cfg( feature = "ssr" )]
#[derive( Clone )]
struct DioxusRenderEndpointState
{
    static_dir: String,
}

#[cfg( feature = "ssr" )]
async fn get_dioxus_render_endpoint( state: DioxusRenderEndpointState ) -> Html<String>
{
    // Get index file.
    let index_html_s = tokio::fs::read_to_string( format!( "{}/index.html", state.static_dir ) )
        .await
        .expect( "Failed to read index.html" );
    let ( index_html_prefix, index_html_suffix ) = index_html_s.split_once( r#"<div id="main">"# ).unwrap();

    let mut app_vdom = VirtualDom::new( frontend::ComponentApp );
    let _ = app_vdom.rebuild();

    let html = dioxus_ssr::render( &app_vdom );
    Html(format!(r#"{index_html_prefix}<div id="main">{html}{index_html_suffix}"#))
}

#[tokio::main]
pub async fn start_server( addr: &str, port: u16, static_dir: &str, assets_dir: &str )
{
    let br_compression = CompressionLayer::new().br( true ).no_gzip().no_deflate();

    // Api router.
    let api_router = Router::new().route( "/hello", get( hello ).layer( br_compression.clone() ) );

    // Main router.
    let mut app = Router::new().nest( "/api", api_router );

    #[cfg( feature = "ssr" )]
    {
        // Robot.txt file get service.
        let robots_file = get_service( ServeFile::new( format!( "{}/robots.txt", assets_dir ) ) )
            .handle_error( handle_error );

        // Favicon.ico file get service.
        let favicon_file = get_service( ServeFile::new( format!( "{}/favicon.ico", assets_dir ) ) )
            .handle_error( handle_error );

        // Static files directory get service.
        let serve_static_dir =
            get_service( ServeDir::new( static_dir ).precompressed_br() ).handle_error( handle_error );

        // Assets files directory get service.
        let serve_assets_dir =
            get_service( ServeDir::new( assets_dir ).precompressed_br() ).handle_error( handle_error );

        // Routes.
        app = app
            .route(
                "/",
                get( {
                    let state = DioxusRenderEndpointState { static_dir: static_dir.into() };
                    move || get_dioxus_render_endpoint( state )
                } ).layer( br_compression.clone() ),
            )
            .route( "/robots.txt", robots_file )
            .route( "/favicon.ico", favicon_file )
            .nest_service( "/static", serve_static_dir.clone() )
            .nest_service( "/assets", serve_assets_dir.clone() );
    }

    // Http tracing logs middleware layer.
    app = logger::middleware_http_tracing( app );

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

async fn handle_error( _err: std::io::Error ) -> impl IntoResponse
{
    ( http::StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong..." )
}

async fn hello() -> impl IntoResponse { "hello from the backend!" }
