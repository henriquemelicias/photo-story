use crate::features::photos;

use axum::{routing::get, Router};

pub fn create_route() -> Router { Router::new().route( "/", get( photos::query_photos ) ) }
