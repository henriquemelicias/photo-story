use std::str::FromStr;

pub use logger::middleware_http_tracing;
use monitoring::logger;

use crate::settings;

pub fn init( log_level: &str ) -> ( Option<logger::WorkerGuard>, Option<logger::WorkerGuard> )
{
    let mut log_output_types = Vec::new();

    if settings::LOGGER.get().unwrap().is_stdout_emitted
    {
        log_output_types.push( logger::OutputType::Stdout );
    }

    if settings::LOGGER.get().unwrap().is_file_emitted
    {
        log_output_types.push( logger::OutputType::File {
            app_name:  &settings::GENERAL.get().unwrap().app_name,
            directory: settings::LOGGER
                .get()
                .unwrap()
                .files_directory
                .as_ref()
                .expect( "Failed to get logger files directory" ),
            prefix:    settings::LOGGER
                .get()
                .unwrap()
                .files_prefix
                .as_ref()
                .expect( "Failed to get logger files prefix" ),
        } )
    }

    logger::init(
        &logger::Level::from_str( log_level ).expect( "Failed to parse log level" ),
        &log_output_types,
    )
}
