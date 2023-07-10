#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

use std::{env, path::PathBuf};
use std::cell::OnceCell;
use std::sync::OnceLock;

use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The error type for the settings module.
#[derive(Error, Debug)]
pub enum Error
{
    /// Failed to get settings from OnceCell.
    #[error( "Failed to get settings from global once_cell variable: {0}. Value is None." )]
    GetIsNone( String ),
    /// Path to the configs directory is invalid.
    #[error( "Path to the configs directory ({0}) is invalid: {1}" )]
    InvalidConfigsDir( PathBuf, &'static str ),
    /// Failed to load settings with figment.
    #[error( "Failed to load settings with figment." )]
    LoadFailure( #[from] figment::Error ),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde( rename_all = "lowercase" )]
pub enum RuntimeEnvironmentType
{
    Development,
    Production,
}

impl Default for RuntimeEnvironmentType
{
    fn default() -> Self { Self::Development }
}

impl std::fmt::Display for RuntimeEnvironmentType
{
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result
    {
        match self
        {
            Self::Development => write!( f, "development" ),
            Self::Production => write!( f, "production" ),
        }
    }
}

impl From<&str> for RuntimeEnvironmentType
{
    fn from( env: &str ) -> Self
    {
        match env.to_lowercase().as_str()
        {
            "production" | "prod" => Self::Production,
            _ => Self::Development,
        }
    }
}

/// Get the path to the configuration files.
///
/// It gets the path from the `cli_args`, environment variable, or uses the default value. The former overwrites the latter.
///
/// # Arguments
///
/// * `default_path` - The default value for the path to the configuration files.
/// * `env_key` - The environment variable key with the path to the configuration files.
/// * `cli_arg` - The command line argument with the path to the configuration files.
///
/// # Errors
///
/// If the path is invalid, then the function returns an error.
///
pub fn get_configs_dir_path( default_path: &str, env_key: &str, cli_arg: &Option<PathBuf> ) -> Result<PathBuf, Error>
{
    // Get the path to the configuration files.
    let configs_dir = env::var( env_key ).unwrap_or_else(|_| default_path.to_string());
    // Substitute the default or environment variable value with the value from the command line.
    let configs_dir = cli_arg.as_ref().map_or_else(
        || PathBuf::from( &configs_dir ),
        |configs_dir| PathBuf::from( configs_dir.as_path() ),
    );

    if !configs_dir.exists()
    {
        return Err( Error::InvalidConfigsDir( configs_dir, "Error: Path does not exist." ) );
    }

    if !configs_dir.is_dir()
    {
        return Err( Error::InvalidConfigsDir( configs_dir, "Error: Path is not a directory." ) );
    }

    Ok( configs_dir )
}

/// Trait for struct that can get the value from a settings struct.
pub trait SettingsGetter<T> {
    /// Get the value from `&self`.
    ///
    /// # Errors
    ///
    /// If the get failed or returns `None`, then return `Error::GetIsNone`.
    fn get( &self ) -> Result<&T, Error>;
}

impl<T> SettingsGetter<T> for OnceCell<T>
{
    fn get( &self ) -> Result<&T, Error>
    {
        self.get().ok_or_else( || Error::GetIsNone(std::any::type_name::<T>().to_string()) )
    }
}

impl<T> SettingsGetter<T> for OnceLock<T>
{
    fn get( &self ) -> Result<&T, Error>
    {
        self.get().ok_or_else( || Error::GetIsNone(std::any::type_name::<T>().to_string()) )
    }
}

/// Get the value from the struct that implements the trait `GetFnParameter<T>`.
/// This function is used to get the settings from the global variable and for it to be more convenient to use.
///
/// # Arguments
///
/// * `cell` - The struct that implements the trait `GetFnParameter<T>`.
///
/// # Errors
///
/// If the get failed or returns `None`, then return `Error::GetIsNone`.
#[inline]
pub fn get<T, U: SettingsGetter<T>>(cell: &U) -> Result<&T, Error>
{
    cell.get()
}

/// Used with the `FigmentImporter` trait generic parameter `U` if there's no command line arguments to overwrite the settings.
#[derive(Serialize)]
pub struct NoCliArgs;

/// Trait to import settings from different sources using figment.
///
/// # Arguments
///
/// * `T` - The type of the settings struct where data will be imported to.
/// * `U` - The type of the command line arguments struct.
///
pub trait FigmentImporter<T: Default + Serialize + Deserialize<'static>, U: Serialize>
{
    /// Import settings from different sources using figment.
    /// The order of priority is as follows:
    /// 1. Default settings.
    /// 2. Load settings from file.
    /// 3. Profile is the environment variables.
    /// 4. Profile is the command line arguments.
    ///
    /// # Arguments
    ///
    /// * `runtime_environment` - The runtime environment.
    /// * `file_path` - The path to the file with settings.
    /// * `env_prefix` - The prefix for environment variables.
    /// * `cli_args` - The command line arguments.
    ///
    /// # Errors
    ///
    /// If the import failed, then return `figment::Error`.
    fn import(
        runtime_environment: Option<&RuntimeEnvironmentType>,
        file_path: Option<&str>,
        env_prefix: Option<&str>,
        cli_args: Option<&U>,
    ) -> Result<T, Error>
    {
        let mut figment = Figment::new();

        // Default settings.
        figment = figment.merge( Serialized::defaults( T::default() ) );

        // Profile is the runtime environment.
        if let Some( run_env ) = runtime_environment
        {
            let run_env = run_env.to_string();
            figment = figment.select( run_env );
        }

        // Load settings from file.
        if let Some( file_path ) = file_path
        {
            let profile = figment.profile().clone().to_string();
            let figment_file = Figment::new().merge( Toml::file( file_path ) ).select( &profile );

            // If default top level key is found, merge it first.
            if figment_file.find_value( "default" ).is_ok()
            {
                figment = figment.merge( figment_file.focus( "default" ) );
            }

            figment = figment.merge( figment_file.focus( &profile ) );
        }

        // Load settings from environment variables.
        if let Some( env_prefix ) = env_prefix
        {
            figment = figment.merge( Env::prefixed( env_prefix ) );
        }

        // Load settings from command line arguments.
        if let Some( cli_args ) = cli_args
        {
            figment = figment.merge( Serialized::defaults( cli_args ) );
        }

        // Extract settings to struct T.
        Ok( figment.extract::<T>()? )
    }
}