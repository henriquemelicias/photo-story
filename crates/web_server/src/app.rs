use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};
use std::path::{Path, PathBuf};

use axum::{
    body::Body,
    extract::State,
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

#[cfg(feature = "ssr")]
use axum::handler::Handler;
use url::Url;

use settings::validators;

#[cfg(feature = "ssr")]
use crate::ssr;

#[derive(Debug, Clone)]
struct ReverseProxyState
{
    client: hyper::client::Client<HttpConnector, Body>,
    api_url: Url,
}

/// Create the main router.
pub async fn create(static_dir: validators::DirectoryPath, assets_dir: validators::DirectoryPath, api_url: Url ) -> Option<Router>
{
    // Main router.
    let mut app = Router::new();

    // Shared state.
    let reverse_proxy_state = ReverseProxyState
    {
        client: hyper::Client::builder().build( HttpConnector::new() ),
        api_url
    };

    let api_router = Router::new().fallback( api_reverse_proxy_handler ).with_state(reverse_proxy_state);
    app = app.nest( "/api", api_router );

    // Robot.txt file get service.
    let robots_path = assets_dir.as_ref().join( "robots.txt" );
    let robots_file = get_service( ServeFile::new( robots_path ) );

    // Favicon.ico file get service.
    let favicon_path = assets_dir.as_ref().join( "favicon.ico" );
    let favicon_file = get_service( ServeFile::new( favicon_path ) );

    // Static files directory get service.
    let serve_static_dir =
        get_service( ServeDir::new( &static_dir ).precompressed_br() );

    // Assets files directory get service.
    let serve_assets_dir =
        get_service( ServeDir::new( &assets_dir ).precompressed_br() );

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
        app = ssr( app, static_dir.as_ref() ).await;
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
async fn ssr( mut app: Router, static_dir: &Path ) -> Router
{
    let dioxus_state = ssr::get_dioxus_render_state( static_dir ).await;
    let dioxus_renderer = ssr::dioxus_render_endpoint.with_state( dioxus_state );
    app = app.fallback_service( dioxus_renderer );

    app
}

/// Reverse proxy requests to the API on the backend.
async fn api_reverse_proxy_handler(State( state ): State<ReverseProxyState>, mut req: Request<Body> )
                                   -> axum::response::Response
{
    let path_query = req.uri().path_and_query().map_or_else( || req.uri().path(), |v| v.as_str() );

    *req.uri_mut() = Uri::builder()
        .scheme( state.api_url.scheme() )
        .authority( state.api_url.authority() )
        .path_and_query( path_query )
        .build()
        .unwrap();

    state.client.request( req ).await.unwrap().into_response()
}

