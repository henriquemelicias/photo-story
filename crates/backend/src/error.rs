use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error
{
    /// Failed to setup the settings.
    #[error( "Failed to setup the settings." )]
    SettingsInitFailed,
    /// The configs directory provided is invalid.
    #[error( "The configs directory provided is invalid." )]
    InvalidConfigsDir,
    /// The log level provided is invalid.
    #[error( "The log level provided ({0}) is invalid." )]
    InvalidLogLevel( &'static str ),

    /// Failed to initialize the server.
    #[error( "Failed to initialize the server." )]
    ServerInitFailed,
}
