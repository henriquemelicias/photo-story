use clap::Parser;
use error_stack::{FutureExt, IntoReport, Report, ResultExt};
use leptos::{leptos_config::Env, LeptosOptions};
use web_server::{logger, settings, Error};

fn main() -> Result<(), Report<Error>> {
    // Parse the command line arguments.
    let cli_args = settings::CliArgs::parse();

    // Get the environment prefix for the settings variables.
    let env_prefix = cli_args
        .env_prefix
        .clone()
        .unwrap_or_else( || "WEB_SERVER".to_string() )
        .to_uppercase();

    // Get the path to the configuration files.
    let configs_dir = settings::get_configs_dir_path(
        "./configs/frontend/",
        &[&env_prefix, "_CONFIGS_DIR"].concat(),
        &cli_args.configs_dir,
    )
    .change_context( Error::InvalidConfigsDir )?;

    // Initialize settings variables.
    let configs =
        settings::init( configs_dir.as_path(), &env_prefix, &cli_args ).change_context( Error::SettingsInitFailed )?;

    // Tracing logs.
    let ( _maybe_stdio_writer_guard, _maybe_file_writer_guard ) =
        logger::init( &configs.general.app_name, configs.logger );

    let static_dir = configs.server.static_dir.clone();
    let static_dir = static_dir.as_ref().to_str().unwrap();

    let leptos_options = LeptosOptions {
        output_name:  configs.general.app_name.clone(),
        site_root:    String::from( "." ),
        site_pkg_dir: String::from( static_dir ),
        env:          Env::try_from( configs.general.run_env.to_string() ).unwrap(),
        site_addr:    configs.server.sock_addr_v4.into(),
        reload_port:  3001,
    };

    tracing::info!( "Starting server for {}.", &configs.general.app_name );
    web_server::init_server( configs.server, leptos_options ).change_context( Error::ServerInitFailed )?;

    Ok( () )
}
