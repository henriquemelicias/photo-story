//! Logger initialization.
//! The logger is based on the `tracing` crate.
//!

pub use logger::middleware_http_tracing;
use monitoring::logger;

use crate::settings;

/// Initializes the logger.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
/// * `logger_settings` - The logger settings [`LoggerConfigs`](settings::LoggerConfigs).
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
pub fn init( app_name: &str, logger_settings: settings::LoggerConfigs ) -> ( Option<logger::WorkerGuard>, Option<logger::WorkerGuard> )
{
    let mut tracing_layers = Vec::new();

    // Emit logs to stdout if the setting is enabled.
    if logger_settings.is_stdout_emitted
    {
        tracing_layers.push( logger::EnableLayer::Stdout );
    }

    let files_emitted_config = logger_settings.files_emitted;

    // Emit logs to file if the setting is enabled.
    if files_emitted_config.is_emitted
    {
        tracing_layers.push( logger::EnableLayer::File {
            app_name,
            directory: files_emitted_config.dir.as_ref(),
            prefix:    &files_emitted_config.files_prefix,
        } );
    }

    logger::init(
        &logger_settings.log_level,
        &tracing_layers,
    )
}