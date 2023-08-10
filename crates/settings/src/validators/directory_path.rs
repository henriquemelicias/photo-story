use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    str::FromStr,
};

use inquire::CustomType;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// The path does not exist.
    #[error( "The path does not exist: {0}" )]
    DoesNotExist( PathBuf ),
    /// The path is not a directory.
    #[error( "The path is not a directory: {0}" )]
    NotADirectory( PathBuf ),
}

/// A path that exists and is a directory.
#[derive(Debug, Clone)]
pub struct DirectoryPath {
    path: PathBuf,
}

impl DirectoryPath {
    /// # Panics
    ///
    /// Panics if the user does not insert a valid directory path.
    #[must_use]
    pub fn prompt() -> Self {
        CustomType::<Self>::new( "Please insert a new directory path:" )
            .with_error_message( "The inserted directory path is not valid." )
            .prompt()
            .unwrap_or_else( |err| panic!( "Failed to get the directory path from the user: {err}" ) )
    }
}

impl std::fmt::Display for DirectoryPath {
    fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result { self.path.fmt( f ) }
}

impl FromStr for DirectoryPath {
    type Err = Error;

    fn from_str( s: &str ) -> Result<Self, Self::Err> {
        let path = PathBuf::from( s );

        if !path.exists() {
            return Err( Error::DoesNotExist( path ) );
        }

        if !path.is_dir() {
            return Err( Error::NotADirectory( path ) );
        }

        Ok( Self { path } )
    }
}

impl TryFrom<PathBuf> for DirectoryPath {
    type Error = Error;

    fn try_from( path: PathBuf ) -> Result<Self, Self::Error> {
        if !path.exists() {
            return Err( Error::DoesNotExist( path ) );
        }

        if !path.is_dir() {
            return Err( Error::NotADirectory( path ) );
        }

        Ok( Self { path } )
    }
}

impl AsRef<Path> for DirectoryPath {
    fn as_ref( &self ) -> &Path { &self.path }
}

impl Serialize for DirectoryPath {
    fn serialize<S>( &self, serializer: S ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.path.serialize( serializer )
    }
}

impl<'de> Deserialize<'de> for DirectoryPath {
    fn deserialize<D>( deserializer: D ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = PathBuf::deserialize( deserializer )?;
        Self::try_from( path ).map_err( serde::de::Error::custom )
    }
}
