//! The settings module.
//!
//! This module is responsible for importing the settings from the configuration files, the environment variables and the command line arguments. The latter overwrites the former.
#![allow( unused )]

use std::{
    env,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    ops::Deref,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, Mutex, OnceLock},
};

use axum::http;
use clap::{Args, Parser};
use error_stack::{FutureExt, Report, ResultExt};
use futures::TryFutureExt;
use hyper::Uri;
use serde::{Deserialize, Serialize};
pub use settings::get_configs_dir_path;
use settings::{validators, FigmentExtractor, RuntimeEnvironment};
use thiserror::Error;
use url::Url;

use crate::logger;

/// Command line arguments interface.
#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct CliArgs {
    /// Configs directory.
    /// The directory where the configuration files are located. There is also the env variable FRONTEND_CONFIGS_DIR.
    /// Default: ./configs/frontend/
    #[arg( long = "configs-dir", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub configs_dir: Option<PathBuf>,

    /// Env prefix.
    /// The prefix for the environment variables to import settings.
    #[arg( long = "env-prefix", value_parser, default_value = "FRONTEND" )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub env_prefix: Option<String>,

    #[command( flatten )]
    pub general: CliArgsGeneral,

    #[command( flatten )]
    pub server: CliArgsServer,

    #[command( flatten )]
    pub logger: CliArgsLogger,
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct CliArgsGeneral {
    /// Set runtime environment.
    #[arg(short = 'e', long = "run-env", value_parser = ["development", "production"])]
    #[serde( skip_serializing_if = "Option::is_none" )]
    run_env: Option<String>,
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct CliArgsServer {
    /// Set the listen addr.
    #[arg( short = 'a', long = "addr", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    addr: Option<Ipv4Addr>,

    /// Set the listen port.
    #[arg( short = 'p', long = "port", value_parser = clap::value_parser! (u16).range(1024..) )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    port: Option<u16>,

    /// Set the proxy url for calls to the backend.
    /// Example: http://localhost:5555
    #[arg( long = "proxy-url", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    proxy_url: Option<String>,

    /// Set the static files directory.
    #[arg( short = 's', long = "static-dir", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    static_dir: Option<PathBuf>,

    /// Set the assets files directory.
    #[arg( long = "assets-dir", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    assets_dir: Option<PathBuf>,
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct CliArgsLogger {
    /// Set the log level.
    #[arg( short = 'l', long = "log-level", value_parser = ["trace", "debug", "info", "warn", "error"])]
    #[serde( skip_serializing_if = "Option::is_none" )]
    log_level: Option<String>,
}

/// Error type for the [`init`] function.
#[derive(Error, Debug)]
#[error( "Failed to import the configs of {0}." )]
pub struct InitImportConfigError( &'static str );

/// Setup the global variables with settings.
///
/// The function imports the settings from the configuration files, the environment variables and the command line arguments. The latter overwrites the former.
///
/// # Arguments
///
/// * `cli_args` - The command line arguments.
/// * `env_prefix` - The prefix for the environment variables to import settings.
/// * `configs_dir` - The directory where the configuration files are located.
///
/// # Errors
///
/// If the settings import fails, then the function returns an error.
pub fn init(
    configs_dir: &Path,
    env_prefix: &str,
    cli_args: &CliArgs,
) -> Result<AllConfigs, Report<InitImportConfigError>> {
    // General settings
    let mut general_configs = GeneralConfigs::extract()
        .file( configs_dir.join( "general.toml" ) )
        .env_prefix( &[env_prefix, "_GENERAL_"].concat() )
        .cli( &cli_args.general )
        .call()
        .change_context( InitImportConfigError( "GENERAL" ) )?;

    // Get runtime environment from general settings.
    let runtime_env: RuntimeEnvironment = env::var( [env_prefix, "_GENERAL_RUN_ENV"].concat() ).map_or_else(
        |_| general_configs.run_env.clone(),
        |env| RuntimeEnvironment::from( &*env ),
    );

    // Server settings.
    let server_configs = ServerConfigs::extract()
        .env( &runtime_env )
        .env_prefix( &[env_prefix, "_SERVER_"].concat() )
        .file( configs_dir.join( "server.toml" ) )
        .cli( &cli_args.server )
        .call()
        .change_context( InitImportConfigError( "SERVER" ) )?;

    // Logger settings.
    let logger_configs = LoggerConfigs::extract()
        .env( &runtime_env )
        .env_prefix( &[env_prefix, "_LOGGER_"].concat() )
        .file( configs_dir.join( "logger.toml" ) )
        .cli( &cli_args.logger )
        .call()
        .change_context( InitImportConfigError( "SERVER" ) )?;

    Ok( AllConfigs {
        general: general_configs,
        server:  server_configs,
        logger:  logger_configs,
    } )
}

/// All the settings imported.
pub struct AllConfigs {
    pub general: GeneralConfigs,
    pub server:  ServerConfigs,
    pub logger:  LoggerConfigs,
}

#[derive(Serialize, Deserialize)]
pub struct GeneralConfigs {
    pub app_name: String,
    pub run_env:  RuntimeEnvironment,
}

impl Default for GeneralConfigs {
    fn default() -> Self {
        Self {
            app_name: "frontend".to_string(),
            run_env:  RuntimeEnvironment::Development,
        }
    }
}

impl FigmentExtractor<'_, Self> for GeneralConfigs {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfigs {
    pub sock_addr_v4: SocketAddrV4,
    pub proxy_url:    Url,
    pub static_dir:   validators::DirectoryPath,
    pub assets_dir:   validators::DirectoryPath,
}

impl Default for ServerConfigs {
    fn default() -> Self {
        Self {
            sock_addr_v4: SocketAddrV4::new( Ipv4Addr::new( 127, 0, 0, 1 ), 5556 ),
            proxy_url:    Url::parse( "http://127.0.0.1:5555" ).unwrap(),
            static_dir:   PathBuf::from( "./build/static" ).try_into().unwrap_or_else( |err| {
                println!(
                    "Failed to parse the default value for the server.static_dir. Error: {}",
                    err
                );
                validators::DirectoryPath::prompt()
            } ),
            assets_dir:   PathBuf::from( "./assets" ).try_into().unwrap_or_else( |err| {
                println!(
                    "Failed to parse the default value for the server.assets_dir. Error: {}",
                    err
                );
                validators::DirectoryPath::prompt()
            } ),
        }
    }
}

impl FigmentExtractor<'_, Self> for ServerConfigs {}

#[derive(Serialize, Deserialize)]
pub struct LoggerConfigs {
    pub log_level:         logger::Level,
    pub is_stdout_emitted: bool,
    pub files_emitted:     LoggerFilesEmittedSubconfig,
}

impl Default for LoggerConfigs {
    fn default() -> Self {
        Self {
            log_level:         logger::Level::from_str( "debug" ).unwrap(),
            is_stdout_emitted: true,
            files_emitted:     LoggerFilesEmittedSubconfig::default(),
        }
    }
}

impl FigmentExtractor<'_, Self> for LoggerConfigs {}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoggerFilesEmittedSubconfig {
    pub is_emitted:   bool,
    pub dir:          validators::DirectoryPath,
    pub files_prefix: String,
}

impl Default for LoggerFilesEmittedSubconfig {
    fn default() -> Self {
        Self {
            is_emitted:   true,
            dir:          PathBuf::from( "./logs" ).try_into().unwrap_or_else( |err| {
                panic!(
                    "Default settings used for logger but emitting to files is enabled and the failed to convert logs \
                     directory: {}",
                    err
                )
            } ),
            files_prefix: "frontend.dev".to_string(),
        }
    }
}
