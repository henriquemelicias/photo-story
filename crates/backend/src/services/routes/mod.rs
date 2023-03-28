mod photos;

pub mod api
{
    use super::*;
    use axum::Router;

    pub fn create_route() -> Router { Router::new().nest( "/photos", photos::create_route() ) }
}
