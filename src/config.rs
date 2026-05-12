use crate::error::CliError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub issuer_id: Option<String>,
    pub key_id: Option<String>,
    pub key_path: Option<String>,
}

impl Config {
    pub fn config_dir() -> Result<PathBuf, CliError> {
        let dir = dirs::home_dir()
            .ok_or_else(|| CliError::Config("cannot determine home directory".into()))?
            .join(".apple-cli");
        Ok(dir)
    }

    pub fn profiles_dir() -> Result<PathBuf, CliError> {
        Ok(Self::config_dir()?.join("profiles"))
    }

    pub fn config_path_for(profile: Option<&str>) -> Result<PathBuf, CliError> {
        match profile {
            Some(name) => Ok(Self::profiles_dir()?.join(format!("{name}.toml"))),
            None => Ok(Self::config_dir()?.join("config.toml")),
        }
    }

    pub fn load(profile: Option<&str>) -> Result<Self, CliError> {
        let path = Self::config_path_for(profile)?;
        if !path.exists() {
            if let Some(name) = profile {
                return Err(CliError::Config(format!(
                    "profile '{name}' not found (create it with `apple-cli config init --profile {name} ...`)"
                )));
            }
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        toml::from_str(&content)
            .map_err(|e| CliError::Config(format!("invalid config file: {e}")))
    }

    pub fn save(&self, profile: Option<&str>) -> Result<PathBuf, CliError> {
        let path = Self::config_path_for(profile)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)
            .map_err(|e| CliError::Config(format!("cannot serialize config: {e}")))?;
        std::fs::write(&path, content)?;
        Ok(path)
    }

    pub fn list_profiles() -> Result<Vec<String>, CliError> {
        let mut profiles = Vec::new();
        let default_path = Self::config_dir()?.join("config.toml");
        if default_path.exists() {
            profiles.push("default".to_string());
        }
        let profiles_dir = Self::profiles_dir()?;
        if profiles_dir.exists() {
            let mut entries: Vec<_> = std::fs::read_dir(&profiles_dir)?
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let name = entry.file_name().to_string_lossy().to_string();
                    name.strip_suffix(".toml").map(String::from)
                })
                .collect();
            entries.sort();
            profiles.extend(entries);
        }
        Ok(profiles)
    }

    pub fn resolve(
        &self,
        cli_issuer_id: Option<&str>,
        cli_key_id: Option<&str>,
        cli_key_path: Option<&str>,
    ) -> ResolvedConfig {
        ResolvedConfig {
            issuer_id: cli_issuer_id
                .map(String::from)
                .or_else(|| std::env::var("APPLE_CLI_ISSUER_ID").ok())
                .or_else(|| self.issuer_id.clone()),
            key_id: cli_key_id
                .map(String::from)
                .or_else(|| std::env::var("APPLE_CLI_KEY_ID").ok())
                .or_else(|| self.key_id.clone()),
            key_path: cli_key_path
                .map(String::from)
                .or_else(|| std::env::var("APPLE_CLI_KEY_PATH").ok())
                .or_else(|| self.key_path.clone()),
        }
    }
}

#[derive(Debug)]
pub struct ResolvedConfig {
    pub issuer_id: Option<String>,
    pub key_id: Option<String>,
    pub key_path: Option<String>,
}

impl ResolvedConfig {
    pub fn require_auth(&self) -> Result<(&str, &str, &str), CliError> {
        let issuer_id = self
            .issuer_id
            .as_deref()
            .ok_or_else(|| CliError::Config("issuer-id not set (use --issuer-id, env APPLE_CLI_ISSUER_ID, or `apple-cli config init`)".into()))?;
        let key_id = self
            .key_id
            .as_deref()
            .ok_or_else(|| CliError::Config("key-id not set (use --key-id, env APPLE_CLI_KEY_ID, or `apple-cli config init`)".into()))?;
        let key_path = self
            .key_path
            .as_deref()
            .ok_or_else(|| CliError::Config("key-path not set (use --key-path, env APPLE_CLI_KEY_PATH, or `apple-cli config init`)".into()))?;
        Ok((issuer_id, key_id, key_path))
    }
}
