//! The main entry point of the frontend.
//!
fn main()
{
    // Server-side rendering.
    #[cfg( feature = "ssr" )]
    #[cfg( target_arch = "wasm32" )]
    dioxus_web::launch_with_props( frontend::presentation::ComponentApp, (), dioxus_web::Config::new().hydrate( true ) );

    // Client-side rendering.
    #[cfg( not( feature = "ssr" ) )]
    #[cfg( target_arch = "wasm32" )]
    dioxus_web::launch( frontend::presentation::ComponentApp );
}
