use std::str::FromStr;
use anyhow::Context;

pub use logger::middleware_http_tracing;
use monitoring::logger;

use crate::settings;

pub fn init( log_level: &str ) -> anyhow::Result<( Option<logger::WorkerGuard>, Option<logger::WorkerGuard> )>
{
    let mut log_output_types = Vec::new();

    if settings::get(&settings::LOGGER )?.is_stdout_emitted
    {
        log_output_types.push( logger::OutputType::Stdout );
    }

    let files_emitted_config = settings::get( &settings::LOGGER )?.files_emitted.clone();

    if files_emitted_config.is_emitted
    {
        log_output_types.push( logger::OutputType::File {
            app_name:  &settings::GENERAL.get().unwrap().app_name,
            directory: &files_emitted_config.dir,
            prefix:    &files_emitted_config.files_prefix,
        } );
    }

    Ok( logger::init(
        &logger::Level::from_str( log_level ).context( "Failed to parse log level" )?,
        &log_output_types,
    ))
}
