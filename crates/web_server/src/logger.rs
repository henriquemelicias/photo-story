pub use logger::middleware_http_tracing;
use monitoring::logger;

use crate::settings;
pub use logger::Level;

/// Initialize the logger.
/// Returns a tuple of stdout and file logger guards. The guards are used to keep the logger alive. If the guards are dropped, the logger will be shut down.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
/// * `logger_settings` - The logger settings [`LoggerConfigs`](settings::LoggerConfigs).
///
/// # Errors
///
/// * If the log level could not be parsed
/// * If failed to get the settings needed to initialize the logger.
///
pub fn init(
    app_name: &str,
    logger_settings: settings::LoggerConfigs,
) -> ( Option<logger::WorkerGuard>, Option<logger::WorkerGuard> )
{
    let mut tracing_layers = Vec::new();

    if logger_settings.is_stdout_emitted
    {
        tracing_layers.push( logger::EnableLayer::Stdout );
    }

    let files_emitted_config = logger_settings.files_emitted;

    if files_emitted_config.is_emitted
    {
        tracing_layers.push( logger::EnableLayer::File {
            app_name,
            directory: files_emitted_config.dir.as_ref(),
            prefix: &files_emitted_config.files_prefix,
        } );
    }

    tracing_layers.push( logger::EnableLayer::SpanTraces );

    logger::init( &logger_settings.log_level, &tracing_layers )
}
