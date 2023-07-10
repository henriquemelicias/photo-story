//! Logger initialization.
//! The logger is based on the `tracing` crate.
//!

use std::str::FromStr;

use anyhow::{Context, Result};

pub use logger::middleware_http_tracing;
use monitoring::logger;

use crate::settings;

/// Initializes the logger.
///
/// # Arguments
///
/// * `log_level` - The log level. The value must be a string representation of `tracing_core::Level`: "ERROR", "WARN", "INFO", "DEBUG", "TRACE". The latter is more verbose than the former.
///
/// # Returns
///
/// The function returns a tuple of two optional `logger::WorkerGuard` objects. The first one is the guard of the stdout logger, the second one is the guard of the file logger.
/// These guards are used to keep the logger workers alive. If the guards are dropped, then the logger workers are stopped. The workers must be kept alive until the end of the program.
///
/// # Errors
///
/// If the logger initialization fails, then the function returns an error.
///
pub fn init( log_level: &str ) -> Result<( Option<logger::WorkerGuard>, Option<logger::WorkerGuard> )>
{
    let mut log_output_types = Vec::new();

    // Emit logs to stdout if the setting is enabled.
    if settings::get( &settings::LOGGER )?.is_stdout_emitted
    {
        log_output_types.push( logger::OutputType::Stdout );
    }

    let files_emitted_config = settings::get( &settings::LOGGER )?.files_emitted.clone();

    // Emit logs to file if the setting is enabled.
    if files_emitted_config.is_emitted
    {
        log_output_types.push( logger::OutputType::File {
            app_name:  &settings::get( &settings::GENERAL )?.app_name,
            directory: &files_emitted_config.dir,
            prefix:    &files_emitted_config.files_prefix,
        } );
    }

    Ok( logger::init(
        &logger::Level::from_str( log_level ).context( "Failed to parse log level" )?,
        &log_output_types,
    ) )
}