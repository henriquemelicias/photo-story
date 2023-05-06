use axum::{http::HeaderValue, Router};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

use crate::{logger, services::routes, settings};

pub async fn create() -> Router
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
                format!(
                    "http://{}:{}",
                    settings::SERVER.get().unwrap().frontend_addr,
                    settings::SERVER.get().unwrap().frontend_port
                )
                .parse::<HeaderValue>()
                .unwrap(),
            ),
        );
    }

    app
}
