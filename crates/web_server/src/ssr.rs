use axum::{
    body::{boxed, Body, BoxBody},
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
    routing::post,
    Router,
};
use frontend::presentation::AppComponent;
use leptos::{view, LeptosOptions};
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn leptos_routes( app: Router<LeptosOptions>, leptos_options: LeptosOptions ) -> Router {
    let routes = generate_route_list( |cx| view! {cx, <AppComponent/>} ).await;
    app.leptos_routes( &leptos_options, routes, |cx| view! {cx, <AppComponent/>} )
        .route( "/leptos/*path", post( leptos_axum::handle_server_fns ) )
        .fallback( file_and_error_handler )
        .with_state( leptos_options )
}

#[axum::debug_handler]
pub async fn file_and_error_handler(
    uri: Uri,
    State( options ): State<LeptosOptions>,
    req: Request<Body>,
) -> axum::response::Response {
    let root = &options.site_root;
    let result = get_static_file( uri, root ).await.unwrap();

    if result.status() == StatusCode::OK {
        result.into_response()
    } else {
        let handler = leptos_axum::render_app_to_stream( options, move |cx| view! {cx, <AppComponent/>} );
        handler( req ).await.into_response()
    }
}

async fn get_static_file( uri: Uri, static_dir: &str ) -> Result<Response<BoxBody>, ( StatusCode, String )> {
    let req = Request::builder().uri( uri.clone() ).body( Body::empty() ).unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new( static_dir ).oneshot( req ).await {
        Ok( response ) => Ok( response.map( boxed ) ),
        Err( err ) => Err( (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!( "Something went wrong: {err}" ),
        ) ),
    }
}
