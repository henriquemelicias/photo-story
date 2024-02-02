use axum::{body::Body, http, http::Request};
use backend::app;
use serde_json::json;
use sqlx::PgPool;
use tower::util::ServiceExt;

#[sqlx::test]
fn photo_add( db: PgPool ) {
    let app = app::create( db );

    let body = json!( {} );

    let request = Request::builder()
        .method( http::Method::POST )
        .uri( "/api/v1/photos" )
        .header( http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref() )
        .body( Body::from( serde_json::to_vec( &body ).unwrap() ) )
        .unwrap();

    let response = app.oneshot( request ).await.unwrap();

    assert_eq!( response.status(), http::StatusCode::OK );
    let response_body = hyper::body::to_bytes( response.into_body() ).await.unwrap();
}
