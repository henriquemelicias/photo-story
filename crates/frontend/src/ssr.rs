use crate::settings;

use axum::{
    body::{Body, StreamBody},
    extract::State,
    handler::Handler,
    http::Request,
    response::IntoResponse,
    routing::get_service,
    Router,
};
use dioxus::prelude::*;
use futures::{stream, StreamExt};
use std::{
    convert::Infallible,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};
use std::error::Error;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};


#[tokio::main]
pub async fn init_server( addr: &str, port: u16 ) -> Result<(), Box<dyn Error>>
{
    let app = app_create().await.ok_or( "Failed to create app" )?;

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

    Ok(())
}

pub async fn app_create() -> Option<Router>
{
    // Main router.
    let mut app = Router::new();

    // Robot.txt file get service.
    let robots_file =
        get_service( ServeFile::new( format!( "{}/robots.txt", settings::SERVER.get()?.assets_dir ) ) );

    // Favicon.ico file get service.
    let favicon_file =
        get_service( ServeFile::new( format!( "{}/favicon.ico", settings::SERVER.get()?.assets_dir ) ) );

    // Static files directory get service.
    let serve_static_dir =
        get_service( ServeDir::new( settings::SERVER.get()?.static_dir.as_str() ).precompressed_br() );

    // Assets files directory get service.
    let serve_assets_dir =
        get_service( ServeDir::new( settings::SERVER.get()?.assets_dir.as_str() ).precompressed_br() );

    let dioxus_state = get_dioxus_render_state( settings::SERVER.get()?.static_dir.as_str() ).await;
    let dioxus_renderer = dioxus_render_endpoint.with_state( dioxus_state );

    // Routes.
    app = app
        .route( "/robots.txt", robots_file )
        .route( "/favicon.ico", favicon_file )
        .nest_service( "/static", serve_static_dir.clone() )
        .nest_service( "/assets", serve_assets_dir.clone() )
        .layer( CompressionLayer::new().br( true ).no_gzip().no_deflate() )
        .fallback_service( dioxus_renderer );

    // Http tracing logs middleware layer.
    app = crate::logger::middleware_http_tracing( app );

    // Compression.
    app = app.layer( CompressionLayer::new().br( true ).no_gzip().no_deflate() );

    // Cors.
    app = app.layer( CorsLayer::permissive() );

    Some( app )
}

#[derive(Clone)]
pub struct DioxusRenderState
{
    index_html_prefix: String,
    index_html_suffix: String,
}

pub async fn get_dioxus_render_state( static_dir: &str ) -> DioxusRenderState
{
    println!( "static_dir: {}", static_dir );
    // Get index file.
    let index_html_s = tokio::fs::read_to_string( format!( "{}/index.html", static_dir ) )
        .await
        .expect( "Failed to read index.html" );

    let ( index_html_prefix, index_html_suffix ) = index_html_s.split_once( r#"<div id="main">"# ).unwrap();

    let mut index_html_prefix = index_html_prefix.to_owned();
    index_html_prefix.push_str( r#"<div id="main">"# );

    let index_html_suffix = index_html_suffix.to_owned();

    DioxusRenderState {
        index_html_prefix,
        index_html_suffix,
    }
}

pub async fn dioxus_render_endpoint( State( state ): State<DioxusRenderState>, _req: Request<Body> )
                                     -> impl IntoResponse
{
    let mut app_vdom = VirtualDom::new( crate::ComponentApp );
    let _ = app_vdom.rebuild();

    let html = dioxus_ssr::render( &app_vdom );

    StreamBody::new(
        stream::once( async move { state.index_html_prefix } )
            .chain( stream::once( async move { html } ) )
            .chain( stream::once( async move { state.index_html_suffix } ) )
            .map( Result::<_, Infallible>::Ok ),
    )
}
