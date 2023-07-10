use anyhow::Context;
use axum::{http::HeaderValue, Router};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

use crate::{logger, services::routes, settings};

#[allow(clippy::unused_async)]
pub async fn create() -> anyhow::Result<Router>
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
        app = app.layer(
            CorsLayer::new().allow_origin(
                format!("http://{}:{}", settings::get( &settings::SERVER )?.frontend_addr, settings::get(&settings::SERVER)?.frontend_port)
                .parse::<HeaderValue>()
                .context( "Unable to parse frontend address and port in the CorsLayer" )?,
            ),
        );
    }

    Ok( app )
}
