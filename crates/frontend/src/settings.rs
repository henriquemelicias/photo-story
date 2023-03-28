#![allow( unused )]

use std::env;
use settings::{ImportFigment, RuntimeEnvironmentType};
use std::sync::Mutex;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

pub static GENERAL: OnceCell<GeneralConfigs> = OnceCell::new();
pub static SERVER: OnceCell<ServerConfigs> = OnceCell::new();
pub static LOGGER: OnceCell<LoggerConfigs> = OnceCell::new();

pub fn setup( server_overwrite_json: String, logger_overwrite_json: String )
{
    let dir_path = "./configs/frontend/";
    let env_prefix = "frontend_";

    let runtime_env = env::var( "FRONTEND_GENERAL_RUN_ENV" ).unwrap_or( "dev".to_string() );
    let runtime_env = RuntimeEnvironmentType::from( runtime_env.as_str() );

    GENERAL.set( GeneralConfigs::import(
        format!( "{}{}", dir_path, "general.toml").as_str(),
        format!( "{}{}", env_prefix, "general_" ).as_str(),
        None,
        None,
    ) );

    SERVER.set( ServerConfigs::import(
        format!( "{}{}", dir_path, "server.toml").as_str(),
        format!( "{}{}", env_prefix, "server_" ).as_str(),
        Some( server_overwrite_json ),
        Some( &runtime_env ),
    ) );
    LOGGER.set( LoggerConfigs::import(
        format!( "{}{}", dir_path, "logger.toml").as_str(),
        format!( "{}{}", env_prefix, "logger_" ).as_str(),
        Some( logger_overwrite_json ),
        Some( &runtime_env )
    ) );
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfigs
{
    pub app_name: String,
    pub about:    String,
    pub run_env:  RuntimeEnvironmentType,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfigs
{
    pub addr:       String,
    pub port:       u16,
    pub static_dir: String,
    pub assets_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct LoggerConfigs
{
    pub log_level:         String,
    pub is_stdout_emitted: bool,
    pub is_file_emitted:   bool,
    pub files_directory:   Option<String>,
    pub files_prefix:      Option<String>,
}

impl ImportFigment<Self> for GeneralConfigs {}
impl ImportFigment<Self> for ServerConfigs {}
impl ImportFigment<Self> for LoggerConfigs {}
