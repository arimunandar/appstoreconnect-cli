use crate::commands::*;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "apple-cli", about = "CLI for Apple App Store Connect API")]
pub struct Cli {
    /// Issuer ID (overrides config and env)
    #[arg(long, global = true, env = "APPLE_CLI_ISSUER_ID")]
    pub issuer_id: Option<String>,

    /// Key ID (overrides config and env)
    #[arg(long, global = true, env = "APPLE_CLI_KEY_ID")]
    pub key_id: Option<String>,

    /// Path to .p8 private key file (overrides config and env)
    #[arg(long, global = true, env = "APPLE_CLI_KEY_PATH")]
    pub key_path: Option<String>,

    /// Named profile to use (loads ~/.apple-cli/profiles/<name>.toml)
    #[arg(long, global = true, env = "APPLE_CLI_PROFILE")]
    pub profile: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Manage CLI configuration
    Config {
        #[command(subcommand)]
        command: config_cmd::ConfigCommands,
    },
    /// List and inspect apps
    Apps {
        #[command(subcommand)]
        command: apps::AppsCommands,
    },
    /// List and inspect builds
    Builds {
        #[command(subcommand)]
        command: builds::BuildsCommands,
    },
    /// Manage beta testers
    #[command(name = "beta-testers")]
    BetaTesters {
        #[command(subcommand)]
        command: beta_testers::BetaTestersCommands,
    },
    /// Manage beta groups
    #[command(name = "beta-groups")]
    BetaGroups {
        #[command(subcommand)]
        command: beta_groups::BetaGroupsCommands,
    },
    /// Manage bundle IDs
    #[command(name = "bundle-ids")]
    BundleIds {
        #[command(subcommand)]
        command: bundle_ids::BundleIdsCommands,
    },
    /// Manage certificates
    Certificates {
        #[command(subcommand)]
        command: certificates::CertificatesCommands,
    },
    /// Manage devices
    Devices {
        #[command(subcommand)]
        command: devices::DevicesCommands,
    },
    /// Manage provisioning profiles
    Profiles {
        #[command(subcommand)]
        command: profiles::ProfilesCommands,
    },
    /// List and inspect App Store versions
    Versions {
        #[command(subcommand)]
        command: versions::VersionsCommands,
    },
    /// Manage users
    Users {
        #[command(subcommand)]
        command: users::UsersCommands,
    },
}
