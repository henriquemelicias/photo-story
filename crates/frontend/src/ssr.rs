use std::{
    convert::Infallible,
    error::Error,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use axum::{
    body,
    body::{Body, BoxBody, StreamBody},
    extract::State,
    handler::Handler,
    http::{Method, Request, Response, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use dioxus::prelude::*;
use futures::{stream, StreamExt};
use hyper::{client::HttpConnector, upgrade::Upgraded, Uri};
use once_cell::sync::Lazy;
use tower::ServiceExt;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

use crate::settings;

type Client = hyper::client::Client<HttpConnector, Body>;

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

    Ok( () )
}

async fn app_create() -> Option<Router>
{
    // Main router.
    let mut app = Router::new();

    /* Api router */
    let client = hyper::Client::builder().build( HttpConnector::new() );

    let api_router = Router::new().fallback( api_reverse_proxy_handler ).with_state( client );
    app = app.nest( "/api", api_router );

    // Robot.txt file get service.
    let robots_file = get_service( ServeFile::new( format!(
        "{}/robots.txt",
        settings::SERVER.get()?.assets_dir
    ) ) );

    // Favicon.ico file get service.
    let favicon_file = get_service( ServeFile::new( format!(
        "{}/favicon.ico",
        settings::SERVER.get()?.assets_dir
    ) ) );

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

async fn api_reverse_proxy_handler( State( client ): State<Client>, mut req: Request<Body> )
    -> axum::response::Response
{
    let path = req.uri().path();
    let path_query = req.uri().path_and_query().map( |v| v.as_str() ).unwrap_or( path );

    let uri = format!( "{}/api{}", settings::SERVER.get().unwrap().proxy_url, path_query );

    *req.uri_mut() = Uri::try_from( uri ).unwrap();

    client.request( req ).await.unwrap().into_response()
}

#[derive(Clone)]
struct DioxusRenderState
{
    index_html_prefix: String,
    index_html_suffix: String,
}

async fn get_dioxus_render_state( static_dir: &str ) -> DioxusRenderState
{
    // Get index file.
    let index_html_s = tokio::fs::read_to_string( format!( "{}/index.html", static_dir ) )
        .await
        .expect( "Failed to read index.html. Did you choose the correct static directory?" );

    let ( index_html_prefix, index_html_suffix ) = index_html_s.split_once( r#"<div id="main">"# ).unwrap();

    let mut index_html_prefix = index_html_prefix.to_owned();
    index_html_prefix.push_str( r#"<div id="main">"# );

    let index_html_suffix = index_html_suffix.to_owned();

    DioxusRenderState {
        index_html_prefix,
        index_html_suffix,
    }
}

async fn dioxus_render_endpoint( State( state ): State<DioxusRenderState>, _req: Request<Body> ) -> impl IntoResponse
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
