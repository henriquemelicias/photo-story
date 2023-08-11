//! The settings module.
//!
//! This module is responsible for importing the settings from the configuration files, the environment variables and the command line arguments. The latter overwrites the former.
#![allow( unused )]

use std::{
    env,
    net::{Ipv4Addr, SocketAddrV4},
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Mutex, OnceLock},
};

use clap::{Args, Parser};
use error_stack::{Report, ResultExt};
use monitoring::logger;
use serde::{Deserialize, Serialize};
pub use settings::get_configs_dir_path;
use settings::{validators, validators::DirectoryPath, FigmentExtractor, RuntimeEnvironment};
use thiserror::Error;
use url::Url;

/// The command line arguments.
#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct CliArgs {
    /// Configs directory.
    /// The directory where the configuration files are located. There is also the env variable BACKEND_CONFIGS_DIR.
    /// Default: ./configs/backend/
    #[arg( long = "configs-dir", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub configs_dir: Option<PathBuf>,

    /// Env prefix.
    /// The prefix for the environment variables to import settings. Is always uppercase.
    #[arg( long = "env-prefix", value_parser, default_value = "BACKEND" )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub env_prefix: Option<String>,

    #[command( flatten )]
    pub general: CliArgsGeneral,

    #[command( flatten )]
    pub server: CliArgsServer,

    #[command( flatten )]
    pub logger: CliArgsLogger,

    #[command( flatten )]
    pub database: CliArgsDatabase,
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

    /// The frontend addr.
    #[arg( long = "frontend-addr", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    frontend_addr: Option<Ipv4Addr>,

    /// The frontend port.
    #[arg( long = "frontend-port", value_parser = clap::value_parser! (u16).range(1024..))]
    #[serde( skip_serializing_if = "Option::is_none" )]
    frontend_port: Option<u16>,
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct CliArgsLogger {
    /// Set the log level.
    #[arg( short = 'l', long = "log-level", value_parser = ["trace", "debug", "info", "warn", "error"])]
    #[serde( skip_serializing_if = "Option::is_none" )]
    log_level: Option<String>,
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct CliArgsDatabase {
    /// Set the database url.
    #[arg( long = "db-url", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    url: Option<String>,

    /// Set the database pool size.
    #[arg( long = "db-pool-size", value_parser = clap::value_parser! (u32).range(1..=100))]
    #[serde( skip_serializing_if = "Option::is_none" )]
    pool_size: Option<u32>,

    #[arg( long = "db-max-lifetime", value_parser = clap::value_parser! (u32).range(1..))]
    #[serde( skip_serializing_if = "Option::is_none" )]
    max_lifetime_minutes: Option<u32>,

    /// Do database migrations on startup.
    #[arg( long = "db-migrate", value_parser )]
    #[serde( skip_serializing_if = "Option::is_none" )]
    do_migration: Option<bool>,
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
/// * `dir_path` - The directory where the configuration files are located.
///
/// # Errors
///
/// If the settings import fails, then the function returns an error.
pub fn init(
    configs_dir: &Path,
    env_prefix: &str,
    cli_args: &CliArgs,
) -> Result<AllConfigs, Report<InitImportConfigError>> {
    // General settings.
    let mut general_configs = GeneralConfigs::extract()
        .file( configs_dir.join( "general.toml" ) )
        .env_prefix( &[env_prefix, "_GENERAL_"].concat() )
        .cli( &cli_args.general )
        .call()
        .change_context( InitImportConfigError( "GENERAL" ) )?;

    // Get runtime environment from general settings.
    let runtime_env: RuntimeEnvironment = env::var( "BACKEND_GENERAL_RUN_ENV" ).map_or_else(
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

    // Database settings.
    let database_configs = DatabaseConfigs::extract()
        .env( &runtime_env )
        .env_prefix( &[env_prefix, "_DATABASE_"].concat() )
        .file( configs_dir.join( "database.toml" ) )
        .cli( &cli_args.database )
        .call()
        .change_context( InitImportConfigError( "DATABASE" ) )?;

    Ok( AllConfigs {
        general:  general_configs,
        server:   server_configs,
        logger:   logger_configs,
        database: database_configs,
    } )
}

/// All the settings imported.
pub struct AllConfigs {
    pub general:  GeneralConfigs,
    pub server:   ServerConfigs,
    pub logger:   LoggerConfigs,
    pub database: DatabaseConfigs,
}

#[derive(Serialize, Deserialize)]
pub struct GeneralConfigs {
    pub app_name: String,
    pub run_env:  RuntimeEnvironment,
}

impl Default for GeneralConfigs {
    fn default() -> Self {
        Self {
            app_name: "backend".to_string(),
            run_env:  RuntimeEnvironment::Development,
        }
    }
}

impl FigmentExtractor<'_, Self> for GeneralConfigs {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfigs {
    pub sock_addr_v4: SocketAddrV4,
    pub frontend_url: Url,
}

impl Default for ServerConfigs {
    fn default() -> Self {
        Self {
            sock_addr_v4: SocketAddrV4::new( Ipv4Addr::new( 127, 0, 0, 1 ), 5555 ),
            frontend_url: Url::parse( "http://127.0.0.1:5556" ).unwrap(),
        }
    }
}

impl FigmentExtractor<'_, Self> for ServerConfigs {}

#[derive(Serialize, Deserialize)]
pub struct LoggerConfigs {
    pub log_level:                logger::Level,
    pub is_stdout_emitted:        bool,
    pub is_tokio_console_emitted: bool,
    pub files_emitted:            LoggerFilesEmittedSubconfig,
}

impl Default for LoggerConfigs {
    fn default() -> Self {
        Self {
            log_level:                logger::Level::from_str( "debug" ).unwrap(),
            is_tokio_console_emitted: true,
            is_stdout_emitted:        true,
            files_emitted:            LoggerFilesEmittedSubconfig::default(),
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
            dir:          PathBuf::from( "./logs2" ).try_into().unwrap_or_else( |err| {
                println!( "Failed to parse the default value for the logger.files_emitted.dir. Error: {err}" );
                validators::DirectoryPath::prompt()
            } ),
            files_prefix: "backend.dev".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseConfigs {
    pub url:                  String,
    pub pool_size:            u32,
    pub max_lifetime_minutes: u32,
    pub do_migration:         bool,
}

impl Default for DatabaseConfigs {
    fn default() -> Self {
        Self {
            url:                  "postgres://postgres:postgres@localhost:5432/photo_story".to_string(),
            pool_size:            10,
            max_lifetime_minutes: 30,
            do_migration:         false,
        }
    }
}

impl FigmentExtractor<'_, Self> for DatabaseConfigs {}
