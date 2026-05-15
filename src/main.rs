mod auth;
mod cli;
mod client;
mod commands;
mod config;
mod error;
mod project;
mod types;

use clap::Parser;
use cli::{Cli, Commands};
use client::ApiClient;
use config::Config;
use error::CliError;
use project::ProjectConfig;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli).await {
        eprintln!("{}", serde_json::to_string_pretty(&e.to_json()).unwrap());
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<(), CliError> {
    let project = ProjectConfig::load()?.unwrap_or_default();

    let profile = cli
        .profile
        .as_deref()
        .or(project.profile.as_deref());

    if let Commands::Config { ref command } = cli.command {
        return commands::config_cmd::execute(command, profile);
    }

    if let Commands::Init { ref app_id } = cli.command {
        return commands::init_cmd::execute(
            profile,
            app_id.as_deref(),
            cli.issuer_id.as_deref(),
            cli.key_id.as_deref(),
            cli.key_path.as_deref(),
        )
        .await;
    }

    let config = Config::load(profile)?;
    let resolved = config.resolve(
        cli.issuer_id.as_deref(),
        cli.key_id.as_deref(),
        cli.key_path.as_deref(),
    );
    let (issuer_id, key_id, key_path) = resolved.require_auth()?;
    let client = ApiClient::new(issuer_id, key_id, key_path);
    let project_app_id = project.app_id.as_deref();

    match cli.command {
        Commands::Config { .. } | Commands::Init { .. } => unreachable!(),
        Commands::Apps { ref command } => commands::apps::execute(command, &client).await,
        Commands::Builds { ref command } => {
            commands::builds::execute(command, &client, project_app_id).await
        }
        Commands::BetaTesters { ref command } => {
            commands::beta_testers::execute(command, &client, project_app_id).await
        }
        Commands::BetaGroups { ref command } => {
            commands::beta_groups::execute(command, &client, project_app_id).await
        }
        Commands::BundleIds { ref command } => {
            commands::bundle_ids::execute(command, &client).await
        }
        Commands::Certificates { ref command } => {
            commands::certificates::execute(command, &client).await
        }
        Commands::Devices { ref command } => commands::devices::execute(command, &client).await,
        Commands::Profiles { ref command } => commands::profiles::execute(command, &client).await,
        Commands::Versions { ref command } => {
            commands::versions::execute(command, &client, project_app_id).await
        }
        Commands::Users { ref command } => commands::users::execute(command, &client).await,
        Commands::CustomerReviews { ref command } => {
            commands::customer_reviews::execute(command, &client, project_app_id).await
        }
        Commands::InAppPurchases { ref command } => {
            commands::in_app_purchases::execute(command, &client, project_app_id).await
        }
        Commands::SubscriptionGroups { ref command } => {
            commands::subscription_groups::execute(command, &client, project_app_id).await
        }
        Commands::Subscriptions { ref command } => {
            commands::subscriptions::execute(command, &client, project_app_id).await
        }
        Commands::SubscriptionLocalizations { ref command } => {
            commands::subscription_localizations::execute(command, &client).await
        }
        Commands::ReviewSubmissions { ref command } => {
            commands::review_submissions::execute(command, &client, project_app_id).await
        }
        Commands::PhasedReleases { ref command } => {
            commands::phased_releases::execute(command, &client).await
        }
        Commands::UserInvitations { ref command } => {
            commands::user_invitations::execute(command, &client).await
        }
        Commands::SandboxTesters { ref command } => {
            commands::sandbox_testers::execute(command, &client).await
        }
        Commands::BundleIdCapabilities { ref command } => {
            commands::bundle_id_capabilities::execute(command, &client).await
        }
        Commands::BetaAppReviewSubmissions { ref command } => {
            commands::beta_app_review_submissions::execute(command, &client).await
        }
    }
}
