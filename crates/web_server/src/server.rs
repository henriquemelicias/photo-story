use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use anyhow::Context;
use axum::{
    body::Body,
    extract::State,
    handler::Handler,
    http::Request,
    response::IntoResponse,
    Router,
    routing::get_service,
};

use hyper::{client::HttpConnector, Uri};
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

use crate::settings;
#[cfg(feature = "ssr")]
use crate::ssr;

type Client = hyper::client::Client<HttpConnector, Body>;

#[tokio::main]
pub async fn init_server( addr: &str, port: u16 ) -> anyhow::Result<()>
{
    let app = app_create().await.context( "Failed to create app." )?;

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

    // Routes.
    app = app
        .route( "/robots.txt", robots_file )
        .route( "/favicon.ico", favicon_file )
        .nest_service( "/static", serve_static_dir )
        .nest_service( "/assets", serve_assets_dir )
        .layer( CompressionLayer::new().br( true ).no_gzip().no_deflate() );

    // Server side rendering.
    #[cfg(feature = "ssr")]
    {
        app = ssr( app ).await;
    }

    // Http tracing logs middleware layer.
    app = crate::logger::middleware_http_tracing( app );

    // Compression.
    app = app.layer( CompressionLayer::new().br( true ).no_gzip().no_deflate() );

    // Cors.
    app = app.layer( CorsLayer::permissive() );

    Some( app )
}

/// Server side rendering.
///
/// # Arguments
///
/// * `app` - The main router.
#[cfg(feature = "ssr")]
async fn ssr( mut app: Router ) -> Router
{
    let dioxus_state = ssr::get_dioxus_render_state( settings::SERVER.get().unwrap().static_dir.as_str() ).await;
    let dioxus_renderer = ssr::dioxus_render_endpoint.with_state( dioxus_state );
    app = app.fallback_service( dioxus_renderer );

    app
}

async fn api_reverse_proxy_handler( State( client ): State<Client>, mut req: Request<Body> )
    -> axum::response::Response
{
    let path = req.uri().path();
    let path_query = req.uri().path_and_query().map_or( path, |v| v.as_str() );

    let uri = &[&settings::SERVER.get().unwrap().proxy_url, "/api", path_query].concat();

    *req.uri_mut() = Uri::try_from( uri ).unwrap();

    client.request( req ).await.unwrap().into_response()
}

