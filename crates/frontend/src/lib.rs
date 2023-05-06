#![allow( non_snake_case )]
#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]
#![feature( async_closure )]

use dioxus::{
    html::{div, text},
    prelude::*,
};
use gloo::net::http::Request;

#[cfg( target_arch = "wasm32" )]
use lol_alloc::{FreeListAllocator, LockedAllocator};
#[cfg( feature = "ssr" )]
#[cfg( not( target_arch = "wasm32" ) )]
pub use non_wasm_ssr::*;

use crate::utils::unwrap_r_abort;
use presentation::{layout, routes};

#[cfg( feature = "ssr" )]
#[cfg( not( target_arch = "wasm32" ) )]
#[path = ""]
mod non_wasm_ssr
{
    pub mod logger;
    pub mod settings;
    pub mod ssr;
}

#[cfg( target_arch = "wasm32" )]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> = LockedAllocator::new( FreeListAllocator::new() );

pub mod domain;
pub mod features;
pub mod infrastructure;
pub mod presentation;
pub mod utils;

// pub fn Layout() -> Html
// {
//     html!(
//         <>
//             <layout::Header />
//
//             <main>
//                 <Switch<routes::Route> render={routes::switch} /> // must be child of <BrowserRouter>
//             </main>
//
//             <layout::Footer />
//
//             <lightbox::modal_view::LightboxModal />
//         </>
//     )
// }

pub fn ComponentApp( cx: Scope ) -> Element
{
    // Make request to api in the backend.
    let test = use_future( cx, (), |_| async move {
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
