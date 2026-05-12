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

    pub fn config_path() -> Result<PathBuf, CliError> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    pub fn load() -> Result<Self, CliError> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        toml::from_str(&content)
            .map_err(|e| CliError::Config(format!("invalid config file: {e}")))
    }

    pub fn save(&self) -> Result<(), CliError> {
        let dir = Self::config_dir()?;
        std::fs::create_dir_all(&dir)?;
        let content = toml::to_string_pretty(self)
            .map_err(|e| CliError::Config(format!("cannot serialize config: {e}")))?;
        std::fs::write(Self::config_path()?, content)?;
        Ok(())
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
