use backend::{logger, settings};

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

    /// Set the listen addr.
    #[clap( long = "frontend-addr" )]
    frontend_addr: Option<String>,

    /// Set the listen port.
    #[clap( short = 'p', long = "frontend-port" )]
    frontend_port: Option<u16>,

    /// Set the log level.
    /// Possible values: trace, debug, info, warn, error.
    #[clap( short = 'l', long = "log-level" )]
    log_level: Option<String>,
}
fn main()
{
    // Initialize global settings variables.
    setup_settings();

    // Tracing logs.
    let ( _maybe_stdio_writer_guard, _maybe_file_writer_guard ) =
        logger::init( settings::LOGGER.get().unwrap().log_level.as_str() );

    tracing::info!( "Starting backend." );

    backend::init_server(
        settings::SERVER.get().unwrap().addr.as_str(),
        settings::SERVER.get().unwrap().port,
    );
}

fn setup_settings()
{
    // Parse the command line arguments.
    let cli_args = CliArgs::parse();

    /* Initialize global settings variables */
    let server_config_overwrite = ServerConfigsOverwrite {
        addr:          cli_args.addr.clone(),
        port:          cli_args.port.clone(),
        frontend_addr: cli_args.frontend_addr.clone(),
        frontend_port: cli_args.frontend_port.clone(),
    };
    let server_config_overwrite = serde_json::to_string( &server_config_overwrite ).unwrap();

    let logger_config_overwrite = LoggerConfigsOverwrite {
        log_level: cli_args.log_level.clone(),
    };
    let logger_config_overwrite = serde_json::to_string( &logger_config_overwrite ).unwrap();

    settings::setup( server_config_overwrite, logger_config_overwrite );
}

#[serde_with::skip_serializing_none]
#[derive(Serialize)]
struct ServerConfigsOverwrite
{
    addr:          Option<String>,
    port:          Option<u16>,
    frontend_addr: Option<String>,
    frontend_port: Option<u16>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize)]
struct LoggerConfigsOverwrite
{
    log_level: Option<String>,
}
