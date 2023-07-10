use std::convert::Infallible;

use axum::{
    body::{Body, StreamBody},
    extract::State,
    http::Request,
    response::IntoResponse,
};
use dioxus::prelude::*;
use futures::{stream, StreamExt};



#[derive(Clone)]
pub struct DioxusRenderState
{
    index_html_prefix: String,
    index_html_suffix: String,
}

pub(crate) async fn get_dioxus_render_state(static_dir: &str ) -> DioxusRenderState
{
    // Get index file.
    let index_html_s = tokio::fs::read_to_string( format!( "{}/index.html", static_dir ) )
        .await
        .expect( "Failed to read index.html. Did you choose the correct static directory?" );

    let ( index_html_prefix, index_html_suffix ) = index_html_s.split_once( r#"<div id="main">"# ).unwrap();

    let mut index_html_prefix = index_html_prefix.to_owned();
    index_html_prefix.push_str( r#"<div id="main">"# );

    let index_html_suffix = index_html_suffix.to_owned();

    DioxusRenderState {
        index_html_prefix,
        index_html_suffix,
    }
}

#[allow( clippy::unused_async )]
pub(crate) async fn dioxus_render_endpoint( State( state ): State<DioxusRenderState>, _req: Request<Body> ) -> impl IntoResponse
{
    let mut app_vdom = VirtualDom::new( frontend::presentation::ComponentApp );
    let _ = app_vdom.rebuild();

    let html = dioxus_ssr::render( &app_vdom );

    StreamBody::new(
        stream::once( async move { state.index_html_prefix } )
            .chain( stream::once( async move { html } ) )
            .chain( stream::once( async move { state.index_html_suffix } ) )
            .map( Result::<_, Infallible>::Ok ),
    )
}
