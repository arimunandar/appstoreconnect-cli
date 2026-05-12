use crate::config::Config;
use crate::error::CliError;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Initialize configuration
    Init {
        /// Issuer ID from App Store Connect
        #[arg(long)]
        issuer_id: String,
        /// Key ID from App Store Connect
        #[arg(long)]
        key_id: String,
        /// Path to .p8 private key file
        #[arg(long)]
        key_path: String,
    },
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// Configuration key (issuer-id, key-id, key-path)
        key: String,
        /// Value to set
        value: String,
    },
    /// List all configured profiles
    List,
}

pub fn execute(cmd: &ConfigCommands, profile: Option<&str>) -> Result<(), CliError> {
    match cmd {
        ConfigCommands::Init {
            issuer_id,
            key_id,
            key_path,
        } => {
            let config = Config {
                issuer_id: Some(issuer_id.clone()),
                key_id: Some(key_id.clone()),
                key_path: Some(key_path.clone()),
            };
            let path = config.save(profile)?;
            let output = serde_json::json!({
                "message": "Configuration saved",
                "profile": profile.unwrap_or("default"),
                "path": path.to_string_lossy(),
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        ConfigCommands::Show => {
            let config = Config::load(profile)?;
            let path = Config::config_path_for(profile)?;
            let output = serde_json::json!({
                "profile": profile.unwrap_or("default"),
                "issuer_id": config.issuer_id,
                "key_id": config.key_id,
                "key_path": config.key_path,
                "config_path": path.to_string_lossy(),
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        ConfigCommands::Set { key, value } => {
            let mut config = Config::load(profile)?;
            match key.as_str() {
                "issuer-id" => config.issuer_id = Some(value.clone()),
                "key-id" => config.key_id = Some(value.clone()),
                "key-path" => config.key_path = Some(value.clone()),
                _ => {
                    return Err(CliError::Config(format!(
                        "unknown config key '{key}' (valid: issuer-id, key-id, key-path)"
                    )));
                }
            }
            config.save(profile)?;
            let output = serde_json::json!({
                "message": format!("Set {key} = {value}"),
                "profile": profile.unwrap_or("default"),
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        ConfigCommands::List => {
            let profiles = Config::list_profiles()?;
            let output = serde_json::json!({ "profiles": profiles });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
