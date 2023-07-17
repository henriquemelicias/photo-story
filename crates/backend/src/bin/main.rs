//! The main entry point of the backend server.
//!
//! This file is the main entry point of the backend server. It is responsible for parsing the command line arguments, initializing the global settings variables, initializing the logger, and starting the server.
//!
use clap::Parser;
use error_stack::{FutureExt, IntoReport, Report, ResultExt};

use backend::{logger, settings, Error};

fn main() -> Result<(), Report<Error>>
{
    // Parse the command line arguments.
    let cli_args = settings::CliArgs::parse();

    // Get the environment prefix for the settings variables.
    let env_prefix = cli_args
        .env_prefix
        .clone()
        .unwrap_or_else( || "BACKEND".to_string() )
        .to_uppercase();

    // Get the path to the configuration files.
    let configs_dir = settings::get_configs_dir_path(
        "./configs/backend/",
        &[&env_prefix, "_CONFIGS_DIR"].concat(),
        &cli_args.configs_dir,
    )
    .into_report()
    .change_context( Error::InvalidConfigsDir )?;

    // Initialize global settings variables.
    let configs =
        settings::init( configs_dir.as_path(), &env_prefix, &cli_args ).change_context( Error::SettingsInitFailed )?;

    // Tracing logs.
    let ( _maybe_stdio_writer_guard, _maybe_file_writer_guard ) =
        logger::init( &configs.general.app_name, configs.logger );

    tracing::info!( "Starting {}", configs.general.app_name );
    backend::init_server( configs.server ).change_context( Error::ServerInitFailed )?;

    Ok( () )
}
