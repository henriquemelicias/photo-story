use std::sync::Arc;

use axum::{Extension, Router};
use tower_http::compression::CompressionLayer;

use crate::{features, infrastructure, logger, presentation::routes};

pub fn create( db: infrastructure::drivers::db::Pool ) -> Router {
    // Main router.
    let mut app = Router::new().nest( "/api/v1", routes::api::create_route() );

    // Services.
    let photos_service = Arc::new( features::photos::Service::new( db ) );

    app = app.layer( Extension( photos_service ) );

    // Http tracing logs middleware layer.
    app = logger::middleware_http_tracing( app );

    // Compression.
    app = app.layer( CompressionLayer::new().br( true ).no_gzip().no_deflate() );

    app
}
