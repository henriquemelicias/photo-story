#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]
#![feature( async_closure )]

use dioxus::prelude::*;
use gloo::net::http::Request;

#[cfg( target_arch = "wasm32" )]
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[cfg( not( feature = "ssr" ) )]
#[cfg( target_arch = "wasm32" )]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> = LockedAllocator::new( FreeListAllocator::new() );

#[cfg( feature = "ssr" )]
#[cfg( not( target_arch = "wasm32" ) )]
pub use non_wasm_ssr::*;

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

pub mod domain;
pub mod features;
pub mod infrastructure;
pub mod presentation;
pub mod utils;

// #[allow( non_snake_case )]
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
