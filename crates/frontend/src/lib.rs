#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]
#![feature( async_closure )]

#[cfg( target_arch = "wasm32" )]
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[cfg( target_arch = "wasm32" )]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> = LockedAllocator::new( FreeListAllocator::new() );

pub mod presentation;

mod domain;
mod features;
mod infrastructure;
mod utils;

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


