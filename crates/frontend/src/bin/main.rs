use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>
{
    #[cfg( feature = "ssr" )]
    {
        #[cfg( target_arch = "wasm32" )]
        dioxus_web::launch_with_props( frontend::ComponentApp, (), dioxus_web::Config::new().hydrate( true ) );

        #[cfg( not( target_arch = "wasm32" ) )]
        {
            use frontend::settings;

            non_wasm_ssr::setup_settings();

            // Tracing logs.
            let ( _maybe_stdio_writer_guard, _maybe_file_writer_guard ) =
                frontend::logger::init( settings::LOGGER.get().unwrap().log_level.as_str() );

            tracing::info!( "Starting frontend." );

            frontend::ssr::init_server( &settings::SERVER.get().unwrap().addr, settings::SERVER.get().unwrap().port )?;
        }
    }

    #[cfg( not( feature = "ssr" ) )]
    #[cfg( target_arch = "wasm32" )]
    dioxus_web::launch( frontend::ComponentApp );

    Ok(())
}

#[cfg( feature = "ssr" )]
#[cfg( not( target_arch = "wasm32" ) )]
mod non_wasm_ssr
{
    use frontend::settings;

    use clap::Parser;
    use serde::Serialize;

    // Command line arguments interface.
    #[derive(Parser, Debug)]
    #[clap()]
    struct CliArgs
    {
        /// Set the listen addr.
        #[clap( short = 'a', long = "addr" )]
        addr: Option<String>,

        /// Set the listen port.
        #[clap( short = 'p', long = "port" )]
        port: Option<u16>,

        /// Set the log level.
        /// Possible values: trace, debug, info, warn, error.
        #[clap( short = 'l', long = "log-level" )]
        log_level: Option<String>,

        /// Set the static files directory
        #[clap( short = 's', long = "static-dir" )]
        static_dir: Option<String>,

        /// Set the assets files directory
        #[clap( long = "assets-dir" )]
        assets_dir: Option<String>,
    }

    #[serde_with::skip_serializing_none]
    #[derive(Serialize)]
    struct ServerConfigsOverwrite
    {
        addr:       Option<String>,
        port:       Option<u16>,
        static_dir: Option<String>,
        assets_dir: Option<String>,
    }

    #[serde_with::skip_serializing_none]
    #[derive(Serialize)]
    struct LoggerConfigsOverwrite
    {
        log_level: Option<String>,
    }

    pub fn setup_settings()
    {
        // Parse the command line arguments.
        let cli_args = CliArgs::parse();

        /* Initialize global settings variables */
        let server_config_overwrite = ServerConfigsOverwrite {
            addr:       cli_args.addr.clone(),
            port:       cli_args.port.clone(),
            static_dir: cli_args.static_dir.clone(),
            assets_dir: cli_args.assets_dir.clone(),
        };
        let server_config_overwrite = serde_json::to_string( &server_config_overwrite ).unwrap();

        let logger_config_overwrite = LoggerConfigsOverwrite {
            log_level: cli_args.log_level.clone(),
        };
        let logger_config_overwrite = serde_json::to_string( &logger_config_overwrite ).unwrap();

        settings::setup( server_config_overwrite, logger_config_overwrite );
    }
}
