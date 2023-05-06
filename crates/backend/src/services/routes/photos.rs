use axum::{routing::get, Router};

use crate::features::photos;

pub fn create_route() -> Router { Router::new().route( "/", get( photos::query_photos ) ) }
