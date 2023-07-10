//! The main entry point of the frontend.
//!
fn main() -> anyhow::Result<()>
{
    // Server-side rendering.
    #[cfg( feature = "ssr" )]
    {
        // Build the frontend.
        #[cfg( target_arch = "wasm32" )]
        dioxus_web::launch_with_props( frontend::presentation::ComponentApp, (), dioxus_web::Config::new().hydrate( true ) );

        // Start the server to serve the static files.
        #[cfg( not( target_arch = "wasm32" ) )]
        {
            use clap::Parser;
            use frontend::{logger, settings};

            // Parse the command line arguments.
            let cli_args = settings::CliArgs::parse();

            // Get the path to the configuration files.
            let configs_dir =
                settings::get_configs_dir_path( "./configs/frontend/", "FRONTEND_CONFIGS_DIR", &cli_args.configs_dir )?;

            // Initialize global settings variables.
            settings::setup( &cli_args, configs_dir.as_path() )?;

            // Tracing logs.
            let ( _maybe_stdio_writer_guard, _maybe_file_writer_guard ) = logger::init(
                settings::get( &settings::LOGGER )?.log_level.as_str(),
            )?;

            tracing::info!("Starting {}", settings::get( &settings::GENERAL )?.app_name);

            frontend::ssr::init_server(
                &settings::get( &settings::SERVER )?.addr,
                settings::get( &settings::SERVER )?.port,
            )?;
        }
    }

    // Client-side rendering.
    #[cfg( not( feature = "ssr" ) )]
    #[cfg( target_arch = "wasm32" )]
    dioxus_web::launch( frontend::presentation::ComponentApp );

    Ok( () )
}
