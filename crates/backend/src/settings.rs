//! The settings module.
//!
//! This module is responsible for importing the settings from the configuration files, the environment variables and the command line arguments. The latter overwrites the former.
//!
#![allow( unused )]

use std::{
    env,
    net::Ipv4Addr,
    path::{Path, PathBuf},
    sync::Mutex,
};
use thiserror::Error;
use std::net::SocketAddrV4;
use std::str::FromStr;
use std::sync::OnceLock;

use clap::Parser;
use url::Url;
use error_stack::{IntoReport, Report, ResultExt};
use serde::{Deserialize, Serialize};
use settings::validators;
use settings::validators::DirectoryPath;
use monitoring::logger;

pub use settings::{get_configs_dir_path};
use settings::{FigmentExtractor, RuntimeEnvironment};

/// The command line arguments.
#[derive(Parser, Debug, Serialize, Deserialize)]
#[clap()]
pub struct CliArgs
{
    /// Set runtime environment.
    #[clap(short = 'e', long = "run-env", value_parser = ["development", "production"])]
    #[serde( skip_serializing_if = "Option::is_none" )]
    run_env: Option<String>,

    /// Set the listen addr.
    #[clap( short = 'a', long = "addr", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    addr: Option<Ipv4Addr>,

    /// Set the listen port.
    #[clap(short = 'p', long = "port", value_parser = clap::value_parser ! (u16).range(1024..65334))]
    #[serde( skip_serializing_if = "Option::is_none" )]
    port: Option<u16>,

    /// The frontend addr.
    #[clap( long = "frontend-addr", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    frontend_addr: Option<Ipv4Addr>,

    /// The frontend port.
    #[clap(long = "frontend-port", value_parser = clap::value_parser ! (u16).range(1024..65335))]
    #[serde( skip_serializing_if = "Option::is_none" )]
    frontend_port: Option<u16>,

    /// Set the log level.
    #[clap(short = 'l', long = "log-level", value_parser = ["trace", "debug", "info", "warn", "error"])]
    #[serde( skip_serializing_if = "Option::is_none" )]
    log_level: Option<String>,

    /// Configs directory.
    /// The directory where the configuration files are located. There is also the env variable BACKEND_CONFIGS_DIR.
    /// Default: ./configs/backend/
    #[clap( long = "configs-dir", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub configs_dir: Option<PathBuf>,

    /// Env prefix.
    /// The prefix for the environment variables to import settings. Is always uppercase.
    #[clap( long = "env-prefix", value_parser, default_value = "BACKEND" )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub env_prefix: Option<String>,
}

/// Error type for the [`init`] function.
#[derive(Error, Debug)]
#[error( "Failed to import the configs of {0}." )]
pub struct InitImportConfigError(&'static str );

/// Setup the global variables with settings.
///
/// The function imports the settings from the configuration files, the environment variables and the command line arguments. The latter overwrites the former.
///
/// # Arguments
///
/// * `cli_args` - The command line arguments.
/// * `env_prefix` - The prefix for the environment variables to import settings.
/// * `dir_path` - The directory where the configuration files are located.
///
/// # Errors
///
/// If the settings import fails, then the function returns an error.
///
pub fn init(configs_dir: &Path, env_prefix: &str, cli_args: &CliArgs ) -> Result<AllConfigs, Report<InitImportConfigError>>
{
    /* General settings */
    let general_path = configs_dir.join( "general.toml" );
    let general_path = if general_path.exists()
    {
        general_path.to_str()
    }
    else
    {
        None
    };

    let mut general_configs = GeneralConfigs::extract(
        None,
        general_path,
        Some( &[env_prefix, "_GENERAL_"].concat() ),
        Some( cli_args ),
    ).into_report().change_context( InitImportConfigError( "GENERAL" ))?;

    // Get runtime environment from general settings.
    let runtime_env: RuntimeEnvironment = env::var( "BACKEND_GENERAL_RUN_ENV" ).map_or_else(
        |_| general_configs.run_env.clone(),
        |env| RuntimeEnvironment::from( &*env ),
    );

    /* Server settings */
    let server_path = configs_dir.join( "server.toml" );
    let server_path = if server_path.exists() { server_path.to_str() } else { None };

    let server_configs = ServerConfigs::extract(
        Some( &runtime_env ),
        server_path,
        Some( &[env_prefix, "_SERVER_"].concat() ),
        Some( cli_args ),
    ).into_report().change_context( InitImportConfigError( "SERVER" ))?;

    /* Logger settings */
    let logger_path = configs_dir.join( "logger.toml" );
    let logger_path = if logger_path.exists()
    {
        logger_path.to_str()
    }
    else
    {
        None
    };

    let mut logger_configs = LoggerConfigs::extract(
        Some( &runtime_env ),
        logger_path,
        Some( &[env_prefix, "_LOGGER_"].concat() ),
        Some( cli_args ),
    ).into_report().change_context( InitImportConfigError( "SERVER" ))?;

    Ok( AllConfigs {
        general: general_configs,
        server:  server_configs,
        logger:  logger_configs,
    } )
}

/// All the settings imported.
pub struct AllConfigs
{
    pub general: GeneralConfigs,
    pub server:  ServerConfigs,
    pub logger:  LoggerConfigs,
}

#[derive(Serialize, Deserialize)]
pub struct GeneralConfigs
{
    pub app_name: String,
    pub run_env: RuntimeEnvironment,
}

impl Default for GeneralConfigs
{
    fn default() -> Self
    {
        Self {
            app_name: "backend".to_string(),
            run_env:  RuntimeEnvironment::Development,
        }
    }
}

impl FigmentExtractor<Self> for GeneralConfigs {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfigs
{
    pub sock_addr_v4:  SocketAddrV4,
    pub frontend_url: Url,
}

impl Default for ServerConfigs
{
    fn default() -> Self
    {
        Self {
            sock_addr_v4: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5555 ),
            frontend_url: Url::parse( "http://127.0.0.1:5556" ).unwrap(),
        }
    }
}

impl FigmentExtractor<Self> for ServerConfigs {}

#[derive(Serialize, Deserialize)]
pub struct LoggerConfigs
{
    pub log_level:          logger::Level,
    pub is_stdout_emitted:  bool,
    pub files_emitted: LoggerFilesEmittedSubconfig,
}

impl Default for LoggerConfigs
{
    fn default() -> Self
    {
        Self {
            log_level:         logger::Level::from_str( "debug").unwrap(),
            is_stdout_emitted: true,
            files_emitted:     LoggerFilesEmittedSubconfig::default()
        }
    }
}

impl FigmentExtractor<Self> for LoggerConfigs {}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoggerFilesEmittedSubconfig
{
    pub is_emitted: bool,
    pub dir: validators::DirectoryPath,
    pub files_prefix: String
}

impl Default for LoggerFilesEmittedSubconfig
{
    fn default() -> Self
    {
        Self {
            is_emitted: true,
            dir: PathBuf::from("./logs2").try_into().unwrap_or_else( |err| {
                println!( "Failed to parse the default value for the logger.files_emitted.dir. Error: {}", err );
                validators::DirectoryPath::prompt()
            }),
            files_prefix: "backend.dev".to_string()
        }
    }
}