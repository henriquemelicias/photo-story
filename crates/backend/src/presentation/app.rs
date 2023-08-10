use std::sync::Arc;

use axum::{http::HeaderValue, Router, Extension};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};
use url::Url;

use crate::{logger, presentation::routes, features, infrastructure};

pub fn create( frontend_url: &Url ) -> Option<Router> {
    // Main router.
    let mut app = Router::new().nest( "/api/v1", routes::api::create_route() );


    // Http tracing logs middleware layer.
    app = logger::middleware_http_tracing( app );

    // Compression.
    app = app.layer( CompressionLayer::new().br( true ).no_gzip().no_deflate() );

    // Cors.
    if cfg!( debug_assertions ) {
        app = app.layer( CorsLayer::permissive() );
    } else {
        app = app.layer( CorsLayer::new().allow_origin( frontend_url.as_str().parse::<HeaderValue>().unwrap() ) );
    }

    Some( app )
}

pub fn init_state( app: Router, db: infrastructure::drivers::db::Pool ) -> Router {
    let photos_service = Arc::new( features::photos::Service::new( db ) );

    app.layer( Extension( photos_service ) )
}
