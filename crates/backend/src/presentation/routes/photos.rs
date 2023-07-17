use axum::{routing::get, Router};

use crate::presentation::by_features::photos::add_photo;

pub fn create_route() -> Router { Router::new().route( "/", get( add_photo::add_photo ) ) }
