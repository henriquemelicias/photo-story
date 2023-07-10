//! The main entry point of the backend server.
//!
//! This file is the main entry point of the backend server. It is responsible for parsing the command line arguments, initializing the global settings variables, initializing the logger, and starting the server.
//!
use clap::Parser;

use backend::{logger, settings};

fn main() -> anyhow::Result<()>
{
    // Parse the command line arguments.
    let cli_args = settings::CliArgs::parse();

    // Get the path to the configuration files.
    let configs_dir =
        settings::get_configs_dir_path( "./configs/backend/", "BACKEND_CONFIGS_DIR", &cli_args.configs_dir )?;

    // Initialize global settings variables.
    settings::setup( &cli_args, configs_dir.as_path() )?;

    // Tracing logs.
    let ( _maybe_stdio_writer_guard, _maybe_file_writer_guard ) = logger::init(
        settings::get( &settings::LOGGER )?.log_level.as_str(),
    )?;

    tracing::info!("Starting {}", settings::get( &settings::GENERAL )?.app_name);

    backend::init_server(settings::get( &settings::SERVER )?.addr.as_str(), settings::get( &settings::SERVER )?.port )?;

    Ok( () )
}
