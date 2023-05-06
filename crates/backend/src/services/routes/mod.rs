mod photos;

pub mod api
{
    use axum::Router;

    use super::*;

    pub fn create_route() -> Router { Router::new().nest( "/photos", photos::create_route() ) }
}
