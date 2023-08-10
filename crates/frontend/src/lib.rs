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
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    use leptos::{mount_to_body, view};

    use crate::presentation::AppComponent;

    mount_to_body( move |cx| {
        view! { cx, <AppComponent/> }
    } );
}
