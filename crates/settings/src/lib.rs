#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

use std::{cell::OnceCell, env, marker::PhantomData, path::PathBuf, sync::OnceLock};

use derive_builder::Builder;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod validators;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde( rename_all = "lowercase" )]
pub enum RuntimeEnvironment {
    Development,
    Production,
}

impl Default for RuntimeEnvironment {
    fn default() -> Self { Self::Development }
}

impl std::fmt::Display for RuntimeEnvironment {
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        match self {
            Self::Development => write!( f, "development" ),
            Self::Production => write!( f, "production" ),
        }
    }
}

impl From<&str> for RuntimeEnvironment {
    fn from( env: &str ) -> Self {
        match env.to_lowercase().as_str() {
            "production" | "prod" => Self::Production,
            _ => Self::Development,
        }
    }
}

/// Error type for the [`get_configs_dir_path`] function.
#[derive(Error, Debug)]
pub enum GetConfigsDirPathError {
    /// The path is not a directory.
    #[error( "The path chosen with the {0} is not a directory: {1}" )]
    NotADirectory( &'static str, PathBuf ),
    /// The directory does not exist.
    #[error( "The directory chosen with the {0} does not exist: {1}" )]
    DirectoryDoesNotExist( &'static str, PathBuf ),
}

/// Get the path to the configuration files.
///
/// It gets the path from the `cli_args`, environment variable with `env_key`, or uses the default value. The former overwrites the latter.
///
/// # Arguments
///
/// * `default_path` - The default value for the path to the configuration files.
/// * `env_key` - The environment variable key with the path to the configuration files.
/// * `cli_arg` - The command line argument with the path to the configuration files.
///
/// # Errors
///
/// If the path is invalid, then the function returns [`GetConfigsDirPathError`].
pub fn get_configs_dir_path(
    default_path: &str,
    env_key: &str,
    cli_arg: &Option<PathBuf>,
) -> Result<PathBuf, GetConfigsDirPathError> {
    let mut path_source = "environment variable";

    // Get the path to the configuration files from the default path or environment variable. The latter overwrites the former.
    let configs_dir = env::var( env_key ).unwrap_or_else( |_| {
        path_source = "default value";
        default_path.to_string()
    } );

    // Substitute the default or environment variable value with the value from the command line.
    let configs_dir = cli_arg.as_ref().map_or_else(
        || PathBuf::from( &configs_dir ),
        |configs_dir| {
            path_source = "command line";
            PathBuf::from( configs_dir.as_path() )
        },
    );

    if !configs_dir.is_dir() {
        return Err( GetConfigsDirPathError::NotADirectory( path_source, configs_dir ) );
    }
    if !configs_dir.exists() {
        return Err( GetConfigsDirPathError::DirectoryDoesNotExist( path_source, configs_dir ) );
    }

    Ok( configs_dir )
}

/// Error type for the function [`SettingsGetter::get`].
#[derive(Error, Debug)]
#[error( "Failed to get settings from the setting struct {0} because the value is None." )]
pub struct SettingsGetError( &'static str );

/// Trait for struct that can get the value from a settings struct.
pub trait SettingsGetter<T> {
    /// Get the value from `T`.
    ///
    /// # Errors
    ///
    /// If the get failed or returns [`None`], then return [`SettingsGetError`].
    fn get( &self ) -> Result<&T, SettingsGetError>;
}

impl<T> SettingsGetter<T> for OnceCell<T> {
    fn get( &self ) -> Result<&T, SettingsGetError> {
        self.get()
            .ok_or_else( || SettingsGetError( std::any::type_name::<T>() ) )
    }
}

impl<T> SettingsGetter<T> for OnceLock<T> {
    fn get( &self ) -> Result<&T, SettingsGetError> {
        self.get()
            .ok_or_else( || SettingsGetError( std::any::type_name::<T>() ) )
    }
}

/// Get the value from the struct that implements the trait [`SettingsGetter`}.
/// This function is used to get the settings from the global variable and for it to be more convenient to use.
///
/// # Arguments
///
/// * `settings` - The struct with the settings.
///
/// # Errors
///
/// If the get failed or returns [`None`], then return [`SettingsGetError`].
#[inline]
pub fn get<T, U: SettingsGetter<T>>( settings: &U ) -> Result<&T, SettingsGetError> { settings.get() }

/// Trait to import settings from different sources using figment.
///
/// # Arguments
///
/// * `T` - The type of the settings struct where data will be imported to.
pub trait FigmentExtractor<'a, T: Default + Serialize + Deserialize<'static>> {
    /// Creates a new extract builder.
    ///
    /// # Builder methods
    ///
    /// - `env()` - The runtime environment.
    /// - `env_prefix()` - The prefix for environment variables.
    /// - `file()` - The path to the file with settings.
    /// - `cli()` - The command line arguments.
    #[must_use]
    fn extract<U: Serialize>() -> ExtractBuilder<'a, T, U> { ExtractBuilder::<T, U>::default() }
}

/// Error type for the function [`FigmentExtractor::extract`].
#[derive(Error, Debug)]
#[error( "Failed to extract settings due to the figment error: {source}." )]
pub struct FigmentExtractionFailedError {
    #[from]
    source: figment::Error,
}

#[derive(Default, Builder)]
#[builder( build_fn( private ), pattern = "owned" )]
pub struct Extract<'a, T: Default + Serialize + Deserialize<'static>, U: Serialize> {
    /// The runtime environment.
    #[builder( setter( into, strip_option ), default )]
    env:          Option<&'a RuntimeEnvironment>,
    /// The prefix for environment variables.
    #[builder( setter( strip_option ), default )]
    env_prefix:   Option<&'a str>,
    /// File path to load settings from. If file does not exist, the setting is ignored.
    #[builder( setter( strip_option ), default )]
    file:         Option<PathBuf>,
    /// The command line arguments.
    #[builder( setter( strip_option ), default )]
    cli:          Option<U>,
    /// The struct type to extract to, this is used to infer the type of the settings.
    #[builder( setter( skip ) )]
    extract_type: PhantomData<T>,
}

impl<T, U> ExtractBuilder<'_, T, U>
where
    T: Default + Serialize + Deserialize<'static>,
    U: Serialize,
{
    /// Import settings from different sources using figment.
    /// The order of priority is as follows:
    /// 1. Default settings.
    /// 2. Load settings from file.
    /// 3. Profile is the environment variables.
    /// 4. Profile is the command line arguments.
    ///
    /// # Errors
    ///
    /// If the import failed, then return [`FigmentExtractionFailedError`].
    pub fn call( self ) -> Result<T, FigmentExtractionFailedError> {
        let data = self.build().expect( "Extract build failed. This should never fail." );

        let mut figment = Figment::new();

        // Profile is the runtime environment.
        if let Some( run_env ) = data.env {
            let run_env = run_env.to_string();
            figment = figment.select( run_env );
        }

        // Load settings from file.
        match data.file {
            Some( file_path ) if file_path.exists() => {
                let profile = figment.profile().clone().to_string();
                let figment_file = Figment::new().merge( Toml::file( file_path ) ).select( &profile );

                // If default top level key is found, merge it first.
                if figment_file.find_value( "default" ).is_ok() {
                    figment = figment.merge( figment_file.focus( "default" ) );
                }

                figment = figment.merge( figment_file.focus( &profile ) );
            }
            // Ignore if file does not exist.
            _ => {}
        }

        // Load settings from environment variables.
        if let Some( env_prefix ) = data.env_prefix {
            figment = figment.merge( Env::prefixed( env_prefix ) );
        }

        // Load settings from command line arguments.
        if let Some( cli_args ) = data.cli {
            figment = figment.merge( Serialized::defaults( cli_args ) );
        }

        // Extract settings to struct T.
        figment.extract::<T>().or_else( |err| {
            // If extraction failed due to missing values try to join with default settings.
            if err.missing() {
                eprintln!(
                    "Failed to extract settings due to missing fields ({}).\nTrying to join with default settings...",
                    err.kind
                );
                figment = figment.join( Serialized::defaults( T::default() ) );
            }

            figment
                .extract::<T>()
                .map_err( |err| FigmentExtractionFailedError { source: err } )
        } )
    }
}
