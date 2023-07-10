#![allow( non_snake_case )]
#![allow( clippy::module_name_repetitions )]

// pub mod models;
// pub mod components;
pub mod layout;
pub mod routes;

use dioxus::prelude::*;
use gloo::net::http::Request;

#[allow( non_snake_case )]
#[must_use]
pub fn ComponentApp( cx: Scope ) -> Element
{
    // Make request to api in the backend.
    let _test = use_future( cx, (), |_| async move {
        Request::get( "/api/v1.0/test" )
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    } );

    cx.render( rsx!(
        layout::ComponentHeader {}
        routes::ComponentRouter {}
        layout::ComponentFooter {}
    ) )
}
