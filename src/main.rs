mod auth;
mod cli;
mod client;
mod commands;
mod config;
mod error;
mod types;

use clap::Parser;
use cli::{Cli, Commands};
use client::ApiClient;
use config::Config;
use error::CliError;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli).await {
        eprintln!("{}", serde_json::to_string_pretty(&e.to_json()).unwrap());
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<(), CliError> {
    let profile = cli.profile.as_deref();

    if let Commands::Config { ref command } = cli.command {
        return commands::config_cmd::execute(command, profile);
    }

    let config = Config::load(profile)?;
    let resolved = config.resolve(
        cli.issuer_id.as_deref(),
        cli.key_id.as_deref(),
        cli.key_path.as_deref(),
    );
    let (issuer_id, key_id, key_path) = resolved.require_auth()?;
    let client = ApiClient::new(issuer_id, key_id, key_path);

    match cli.command {
        Commands::Config { .. } => unreachable!(),
        Commands::Apps { ref command } => commands::apps::execute(command, &client).await,
        Commands::Builds { ref command } => commands::builds::execute(command, &client).await,
        Commands::BetaTesters { ref command } => {
            commands::beta_testers::execute(command, &client).await
        }
        Commands::BetaGroups { ref command } => {
            commands::beta_groups::execute(command, &client).await
        }
        Commands::BundleIds { ref command } => {
            commands::bundle_ids::execute(command, &client).await
        }
        Commands::Certificates { ref command } => {
            commands::certificates::execute(command, &client).await
        }
        Commands::Devices { ref command } => commands::devices::execute(command, &client).await,
        Commands::Profiles { ref command } => commands::profiles::execute(command, &client).await,
        Commands::Versions { ref command } => commands::versions::execute(command, &client).await,
        Commands::Users { ref command } => commands::users::execute(command, &client).await,
    }
}
