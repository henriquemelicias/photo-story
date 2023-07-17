#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]
#![feature( async_closure )]

pub mod presentation;

mod domain;
mod features;
mod infrastructure;
mod utils;
