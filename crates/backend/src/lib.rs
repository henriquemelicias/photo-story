#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

// Expose app Router creation to facilitate e2e tests.
pub use error::Error;
use error_stack::{Report, ResultExt};
pub use presentation::app;
use thiserror::Error;
use tracing::instrument;
mod domain;
mod error;
mod features;
mod infrastructure;
pub mod logger;
mod presentation;
pub mod settings;
mod utils;

#[derive(Error, Debug)]
pub enum InitServerError {
    /// Failed to bind on the provided address.
    #[error( "Failed to bind on the address: {0}" )]
    AddressBindFailed( std::net::SocketAddrV4 ),
    /// Failed to connect to the database.
    #[error( "Failed to connect to the database." )]
    DatabaseConnectionFailed,
    /// Failed to execute migrations on the database.
    #[error( "Failed to execute migrations on the database." )]
    DatabaseMigrationFailed,
    /// The address provided is invalid.
    #[error( "The address provided ({0}) is invalid." )]
    InvalidAddr( &'static str ),
    /// Failed to serve the server.
    #[error( "Failed to serve the server." )]
    ServerServeFailed,
}

#[tokio::main]
#[instrument( name = "APP", err, skip( server_settings, database_settings ) )]
pub async fn init_server(
    server_settings: settings::ServerConfigs,
    database_settings: settings::DatabaseConfigs,
) -> Result<(), Report<InitServerError>> {
    tracing::info!(
        "Initializing server the with settings: [ sock_addr_v4={}, frontend_url={}, database_pool_size={}, \
         database_max_lifetime_minutes={} ].",
        server_settings.sock_addr_v4,
        server_settings.frontend_url.as_str(),
        database_settings.pool_size,
        database_settings.max_lifetime_minutes,
    );

    // Database connection.
    let db = infrastructure::drivers::db::connect(
        &database_settings.url,
        database_settings.pool_size,
        database_settings.max_lifetime_minutes,
    )
    .await
    .change_context( InitServerError::DatabaseConnectionFailed )?;
    tracing::info!( "Connected successfully to database." );

    // Database migration.
    if database_settings.do_migration {
        infrastructure::drivers::db::migrate( &db )
            .await
            .change_context( InitServerError::DatabaseMigrationFailed )?;
        tracing::info!( "Database migrations executed successfully." );
    }

    // Create app router.
    let mut app = presentation::app::create( db );

    // Cors.
    if cfg!( debug_assertions ) {
        app = app.layer( tower_http::cors::CorsLayer::permissive() );
    } else {
        app = app.layer(
            tower_http::cors::CorsLayer::new().allow_origin(
                server_settings
                    .frontend_url
                    .as_str()
                    .parse::<axum::http::HeaderValue>()
                    .unwrap(),
            ),
        );
    }

    // Server.
    let server = axum::Server::try_bind( &server_settings.sock_addr_v4.into() )
        .change_context( InitServerError::AddressBindFailed( server_settings.sock_addr_v4 ) )?;

    tracing::info!( "Server bound to http://{} successfully.", server_settings.sock_addr_v4 );

    server
        .serve( app.into_make_service() )
        .await
        .change_context( InitServerError::ServerServeFailed )?;

    Ok( () )
}
