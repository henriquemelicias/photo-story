#[cfg( feature = "ssr" )]
use dioxus_web::Config;

fn main()
{
    #[cfg( feature = "ssr" )]
    dioxus_web::launch_with_props( frontend::ComponentApp, (), Config::new().hydrate( true ) );

    #[cfg( not( feature = "ssr" ) )]
    dioxus_web::launch( frontend::ComponentApp )
}
