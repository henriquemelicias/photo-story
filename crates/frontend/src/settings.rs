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
use std::sync::OnceLock;

use clap::Parser;
use serde::{Deserialize, Serialize};

pub use settings::{get, get_configs_dir_path, Error};
use settings::{ImportFigment, RuntimeEnvironmentType};

pub static GENERAL: OnceLock<GeneralConfigs> = OnceLock::new();
pub static SERVER: OnceLock<ServerConfigs> = OnceLock::new();
pub static LOGGER: OnceLock<LoggerConfigs> = OnceLock::new();

// Command line arguments interface.
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

    /// Set the proxy url for calls to the backend.
    /// Example: http://localhost:5555
    #[clap( long = "proxy-url", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    proxy_url: Option<String>,

    /// Set the log level.
    #[clap(short = 'l', long = "log-level", value_parser = ["trace", "debug", "info", "warn", "error"])]
    #[serde( skip_serializing_if = "Option::is_none" )]
    log_level: Option<String>,

    /// Set the static files directory.
    #[clap( short = 's', long = "static-dir", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    static_dir: Option<PathBuf>,

    /// Set the assets files directory.
    #[clap( long = "assets-dir", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    assets_dir: Option<PathBuf>,

    /// Configs directory.
    /// The directory where the configuration files are located. There is also the env variable FRONTEND_CONFIGS_DIR.
    /// Default: ./configs/frontend/
    #[clap( long = "configs-dir", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub configs_dir: Option<PathBuf>,
}

/// Setup the global variables with settings.
///
/// The function imports the settings from the configuration files, the environment variables and the command line arguments. The latter overwrites the former.
///
/// # Arguments
///
/// * `cli_args` - The command line arguments.
/// * `configs_dir` - The directory where the configuration files are located.
///
/// # Errors
///
/// If the settings import fails, then the function returns an error.
///
pub fn setup( cli_args: &CliArgs, configs_dir: &Path ) -> Result<(), settings::Error>
{
    /* General settings */
    let general_path = configs_dir.join( "general.toml" );
    let general_path = if general_path.exists() { general_path.to_str() } else { None };

    let mut general_configs = GeneralConfigs::import(
        None,
        general_path,
        Some( "backend_general_" ),
        cli_args,
    )?;

    // Get runtime environment from general settings.
    let runtime_env: RuntimeEnvironmentType = env::var( "BACKEND_GENERAL_RUN_ENV" ).map_or_else(
        |_| general_configs.run_env.clone(),
        |env| RuntimeEnvironmentType::from( &*env ),
    );

    /* Server settings */
    let server_path = configs_dir.join( "server.toml" );
    let server_path = if server_path.exists() { server_path.to_str() } else { None };

    let server_configs = ServerConfigs::import(
        Some( &runtime_env ),
        server_path,
        Some( "backend_server_" ),
        cli_args,
    )?;

    /* Logger settings */
    let logger_path = configs_dir.join( "logger.toml" );
    let logger_path = if logger_path.exists() { logger_path.to_str() } else { None };

    let mut logger_configs = LoggerConfigs::import(
        Some( &runtime_env ),
        logger_path,
        Some( "backend_logger_" ),
        cli_args,
    )?;

    /* Set global settings variables */
    GENERAL.set( general_configs );
    SERVER.set( server_configs );
    LOGGER.set( logger_configs );

    Ok( () )
}

#[derive(Serialize, Deserialize)]
pub struct GeneralConfigs
{
    pub app_name: String,
    pub run_env:  RuntimeEnvironmentType,
}

impl Default for GeneralConfigs
{
    fn default() -> Self
    {
        Self {
            app_name: "frontend".to_string(),
            run_env:  RuntimeEnvironmentType::Development,
        }
    }
}

impl ImportFigment<Self, CliArgs> for GeneralConfigs {}

#[derive(Serialize, Deserialize)]
pub struct ServerConfigs
{
    pub addr:       String,
    pub port:       u16,
    pub proxy_url:  String,
    pub static_dir: String,
    pub assets_dir: String,
}

impl Default for ServerConfigs
{
    fn default() -> Self
    {
        Self {
            addr:       "127.0.0.1".to_string(),
            port:       5556,
            proxy_url:  "http://127.0.0.1:5555".to_string(),
            static_dir: "./target/static".to_string(),
            assets_dir: "./assets".to_string(),
        }
    }
}

impl ImportFigment<Self, CliArgs> for ServerConfigs {}

#[derive(Serialize, Deserialize)]
pub struct LoggerConfigs
{
    pub log_level:          String,
    pub is_stdout_emitted:  bool,
    pub files_emitted: LoggerFilesEmittedSubconfig,
}

impl Default for LoggerConfigs
{
    fn default() -> Self
    {
        Self {
            log_level:         String::from( "debug" ),
            is_stdout_emitted: true,
            files_emitted:     LoggerFilesEmittedSubconfig::default()
        }
    }
}

impl ImportFigment<Self, CliArgs> for LoggerConfigs {}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoggerFilesEmittedSubconfig
{
    pub is_emitted: bool,
    pub dir: String,
    pub files_prefix: String
}

impl Default for LoggerFilesEmittedSubconfig
{
    fn default() -> Self
    {
        Self {
            is_emitted: true,
            dir: "./logs".to_string(),
            files_prefix: "frontend.dev".to_string()
        }
    }
}
