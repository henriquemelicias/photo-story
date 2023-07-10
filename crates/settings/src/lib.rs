#![feature( never_type )]

use std::{env, path::PathBuf};
use std::cell::{OnceCell};
use std::sync::OnceLock;

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
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

#[derive(Serialize, Deserialize, Clone, PartialEq)]
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
            "prod" => Self::Production,
            "production" => Self::Production,
            _ => Self::Development,
        }
    }
}

/// Get the path to the configuration files.
///
/// It gets the path from the cli_args, environment variable, or uses the default value. The former overwrites the latter.
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
    let configs_dir = env::var( env_key ).unwrap_or( default_path.to_string() );
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

/// The type used by the get function for the struct used to store the settings.
pub trait GetFnParameter<T> {
    fn get( &self ) -> Result<&T, Error>;
}

impl<T> GetFnParameter<T> for OnceCell<T>
{
    fn get( &self ) -> Result<&T, Error>
    {
        self.get().ok_or_else( || Error::GetIsNone(std::any::type_name::<T>().to_string()) )
    }
}

impl<T> GetFnParameter<T> for OnceLock<T>
{
    fn get( &self ) -> Result<&T, Error>
    {
        self.get().ok_or_else( || Error::GetIsNone(std::any::type_name::<T>().to_string()) )
    }
}

/// Get the value from the OnceLock. If the value is None, then return an error.
/// This function is used to get the settings from the global variable and for it to be more convenient to use.
#[inline]
pub fn get<T, U: GetFnParameter<T>>(cell: &U) -> Result<&T, Error>
{
    cell.get()
}

/// Trait to import settings from different sources using figment.
///
/// # Arguments
///
/// * `T` - The type of the settings struct where data will be imported to.
/// * `U` - The type of the command line arguments struct.
///
pub trait ImportFigment<T: Default + Serialize + Deserialize<'static>, U: Serialize>
{
    fn import(
        runtime_environment: Option<&RuntimeEnvironmentType>,
        file_path: Option<&str>,
        env_prefix: Option<&str>,
        cli_args: &U,
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
        figment = figment.merge( Serialized::defaults( cli_args ) );

        // Extract settings to struct T.
        Ok( figment.extract::<T>()? )
    }
}

/// Trait to import settings from different sources using figment and without CLI overwrites.
///
/// # Arguments
///
/// * `T` - The type of the settings struct where data will be imported to.
///
pub trait ImportFigmentWithoutCli<T: Default + Serialize + Deserialize<'static>>
{
    fn import(
        runtime_environment: Option<&RuntimeEnvironmentType>,
        file_path: Option<&str>,
        env_prefix: Option<&str>,
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

        // Extract settings to struct T.
        Ok( figment.extract::<T>()? )
    }
}
