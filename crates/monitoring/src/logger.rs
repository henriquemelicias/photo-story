//! Tracing logger.
//!
//! Provides a tracing logger that can be used to log events and spans to user enabled options:
//! * stdout
//! * file
//! * wasm
//!
//! # Examples
//!
//! ```
//! # use std::path::Path;
//! use monitoring::logger::{self, Level, EnableLayer};
//!
//! // Initialize the logger with the desired options. The guards returned by this function must be
//! // kept alive for the duration of the program.
//! let ( _maybe_stdout_writer_guard, _maybe_file_writer_guard  ) = logger::init(
//!     &Level::INFO,
//!     &vec![ EnableLayer::Stdout, EnableLayer::File { app_name: "monitoring", directory: Path::new("../../logs"), prefix: "doc.tests"} ],
//! );
//! ```

use std::error::Error;
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use axum::{
    body::{Body, BoxBody, Bytes},
    http::{HeaderMap, Request, Response},
    Router,
};
use tower_http::{classify::ServerErrorsFailureClass, trace as http_trace};
use tracing::Span;
pub use tracing_appender::non_blocking::WorkerGuard;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Wrapper enum for tracing level.
pub struct Level( tracing::Level );

/// Error that can occur when parsing a `Level` from a string.
#[derive(Debug)]
pub struct LevelParseError( String );

impl Display for LevelParseError
{
    fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
    {
        write!( f, "Failed to parse level: {}", self.0 )
    }
}

impl Error for LevelParseError {}

impl FromStr for Level
{
    type Err = LevelParseError;

    fn from_str( s: &str ) -> Result<Self, Self::Err>
    {
        let level = s.to_uppercase();
        match level.as_str()
        {
            "TRACE" => Ok( Self( tracing::Level::TRACE ) ),
            "DEBUG" => Ok( Self( tracing::Level::DEBUG ) ),
            "INFO"  => Ok( Self( tracing::Level::INFO  ) ),
            "WARN"  => Ok( Self( tracing::Level::WARN  ) ),
            "ERROR" => Ok( Self( tracing::Level::ERROR ) ),
            _       => Err( LevelParseError( level ) ),
        }
    }
}

impl Serialize for Level
{
    fn serialize<S>( &self, serializer: S ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str( &self.0.to_string() )
    }
}

impl<'de> Deserialize<'de> for Level
{
    fn deserialize<D>( deserializer: D ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize( deserializer )?;
        Self::from_str( &s ).map_err( |err| serde::de::Error::custom( err.to_string() ) )
    }
}

/// Layers that can be enabled for the tracing.
#[derive(Debug)]
pub enum EnableLayer<'a>
{
    /// Output to a file.
    File
    {
        /// The name of the application.
        app_name:  &'a str,
        /// The directory to create the log files.
        directory: &'a Path,
        /// The prefix to use for the file name.
        prefix:    &'a str,
    },
    /// Output to stdout.
    Stdout,
    /// Output to wasm ( console.log ).
    Wasm,
    /// Enable capture `SpanTraces` .
    SpanTraces,
}

/// Initializes the logger with the given options.
///
/// # Arguments
///
/// * `level_filter` - The filter to use. Any filter level below this will be ignored.
/// * `output_types` - The outputs to use on the tracing layers.
/// * `input_types` - The inputs to use on the tracing layers.
///
/// # Returns
///
/// A tuple of the stdout writer guard and the file writer guard. These guards must be kept alive
/// for the duration of the program.
///
/// # Examples
///
/// see [`crate::logger`] for an example.
///
pub fn init(
    level_filter: &Level,
    layers_enabled: &Vec<EnableLayer>,
) -> ( Option<WorkerGuard>, Option<WorkerGuard> )
{
    // Use tracing::level
    let level_filter = level_filter.0;

    // Layers to be used.
    let mut layers = Vec::new();

    let mut guard_io_writer = None;
    let mut guard_file_writer = None;

    // Check output_types_enabled.
    for output_type in layers_enabled
    {
        match output_type
        {
            // Write logs to stdout.
            EnableLayer::Stdout =>
            {
                let ( non_blocking_io_writer, guard ) = tracing_appender::non_blocking( std::io::stdout() );
                let stdout_layer = tracing_subscriber::fmt::layer().with_writer( non_blocking_io_writer );

                guard_io_writer = Some( guard );
                layers.push( stdout_layer.boxed() );
            }
            // Write logs to local file.
            EnableLayer::File {
                app_name,
                directory,
                prefix,
            } =>
            {
                let file_appender = tracing_appender::rolling::hourly( *directory, prefix );
                let ( non_blocking_file_writer, guard ) = tracing_appender::non_blocking( file_appender );

                // TODO: Change into tracing_subscriber::fmt::format::Json when stable.
                let file_layer = BunyanFormattingLayer::new( (*app_name).to_string(), non_blocking_file_writer );

                guard_file_writer = Some( guard );
                layers.push( file_layer.boxed() );
            }
            // Write logs to wasm console.
            EnableLayer::Wasm =>
            {
                layers.push( tracing_wasm::WASMLayer::default().boxed() );
            }

            // Capture SpanTraces.
            EnableLayer::SpanTraces =>
                {
                    layers.push( tracing_error::ErrorLayer::default().boxed() );
                }
        }
    }

    // Log level filter.
    let log_level_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else( |_| tracing_subscriber::EnvFilter::new( level_filter.as_str() ) );

    // Register layers to registry.
    tracing_subscriber::Registry::default()
        .with( layers )
        .with( log_level_filter )
        .init();

    tracing::info!( "Initialized logging with instrumentation. Settings used: level_filter={level_filter}, layers={layers_enabled:?}." );

    ( guard_io_writer, guard_file_writer )
}

/// Adds tracing instrumentation to the given router. This will add tracing to all http requests and responses.
///
/// # Arguments
///
/// * `router` - The router to add the instrumentation to.
///
/// # Returns
///
/// The router with the instrumentation added.
///
pub fn middleware_http_tracing( router: Router ) -> Router
{
    let trace_layer = http_trace::TraceLayer::new_for_http()
        .make_span_with( |_request: &Request<Body>| {
            let request_id = Uuid::new_v4().to_string();
            tracing::info_span!("HTTP", %request_id)
        } )
        .on_request( |request: &Request<Body>, _span: &Span| {
            tracing::debug!( "REQUEST{{method={}, path={}}}", request.method(), request.uri().path() );
        } )
        .on_response( |response: &Response<BoxBody>, latency: Duration, _span: &Span| {
            tracing::debug!( "RESPONSE{{status={}, latency={:?}}}", response.status(), latency );
        } )
        .on_body_chunk( |chunk: &Bytes, _latency: Duration, _span: &Span| {
            tracing::debug!( "BODY{{bytes={}}}", chunk.len() );
        } )
        .on_eos(
            |_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
                tracing::debug!( "EOS{{stream_closed_after={:?}}}", stream_duration );
            },
        )
        .on_failure( |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
            tracing::error!( "ERROR{{{}}}", error );
        } );

    router.layer( trace_layer )
}
