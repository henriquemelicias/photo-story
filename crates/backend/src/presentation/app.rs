use axum::http::HeaderValue;
use axum::Router;
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

use crate::presentation::routes;
use crate::logger;
use url::Url;

#[allow(clippy::unused_async)]
pub async fn create( frontend_url: Url ) -> Option<Router>
{
    // Main router.
    let mut app = Router::new().nest( "/api/v1", routes::api::create_route() );

    // Http tracing logs middleware layer.
    app = logger::middleware_http_tracing( app );

    // Compression.
    app = app.layer( CompressionLayer::new().br( true ).no_gzip().no_deflate() );

    // Cors.
    if cfg!( debug_assertions )
    {
        app = app.layer( CorsLayer::permissive() );
    }
    else
    {
        app = app.layer( CorsLayer::new().allow_origin( frontend_url.as_str().parse::<HeaderValue>().unwrap() ) );
    }

    Some( app )
}
