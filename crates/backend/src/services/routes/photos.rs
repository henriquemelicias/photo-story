use axum::{Router, routing::get};

use crate::features::photos;

pub fn create_route() -> Router { Router::new().route( "/", get( photos::query_photos ) ) }
