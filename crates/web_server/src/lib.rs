#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

pub mod logger;
pub mod settings;
pub mod server;

#[cfg(feature = "ssr")]
mod ssr;