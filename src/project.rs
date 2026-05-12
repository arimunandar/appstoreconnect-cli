use crate::error::CliError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectConfig {
    pub profile: Option<String>,
    pub app_id: Option<String>,
    pub app_name: Option<String>,
    pub bundle_id: Option<String>,
}

impl ProjectConfig {
    pub fn dir() -> PathBuf {
        PathBuf::from(".apple")
    }

    pub fn path() -> PathBuf {
        Self::dir().join("config.toml")
    }

    pub fn load() -> Result<Option<Self>, CliError> {
        let path = Self::path();
        if !path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(&path)?;
        let config: Self = toml::from_str(&content)
            .map_err(|e| CliError::Config(format!("invalid .apple/config.toml: {e}")))?;
        Ok(Some(config))
    }

    pub fn save(&self) -> Result<(), CliError> {
        let dir = Self::dir();
        std::fs::create_dir_all(&dir)?;
        let content = toml::to_string_pretty(self)
            .map_err(|e| CliError::Config(format!("cannot serialize project config: {e}")))?;
        std::fs::write(Self::path(), content)?;
        Ok(())
    }

    pub fn ensure_gitignore() -> Result<(), CliError> {
        let gitignore = PathBuf::from(".gitignore");
        let entry = ".apple/";

        if gitignore.exists() {
            let content = std::fs::read_to_string(&gitignore)?;
            if content.lines().any(|line| line.trim() == entry) {
                return Ok(());
            }
            let sep = if content.ends_with('\n') { "" } else { "\n" };
            std::fs::write(&gitignore, format!("{content}{sep}{entry}\n"))?;
        } else {
            std::fs::write(&gitignore, format!("{entry}\n"))?;
        }
        Ok(())
    }

    pub fn generate_claude_skill(app_name: Option<&str>, app_id: Option<&str>) -> Result<(), CliError> {
        let skill_dir = PathBuf::from(".claude").join("commands");
        std::fs::create_dir_all(&skill_dir)?;

        let app_context = match (app_name, app_id) {
            (Some(name), Some(id)) => format!("This project is linked to the App Store Connect app **{name}** (ID: `{id}`).\n\n"),
            (None, Some(id)) => format!("This project is linked to App Store Connect app ID `{id}`.\n\n"),
            _ => String::new(),
        };

        let skill_content = format!(r#"# Apple Developer CLI

{app_context}The project has a local `.apple/config.toml` that stores the App Store Connect profile and app ID. Commands like `builds list`, `beta-groups list`, and `versions list` automatically use the configured app ID.

## Quick Reference

```sh
# Builds
apple-cli builds list                    # list recent builds
apple-cli builds list --all              # list all builds
apple-cli builds list --limit 5          # last 5 builds

# TestFlight
apple-cli beta-groups list               # list beta groups
apple-cli beta-testers list              # list testers
apple-cli beta-testers add \
  --email USER@EMAIL \
  --first-name FIRST \
  --last-name LAST \
  --beta-group-ids GROUP_ID              # add tester

# App Store Versions
apple-cli versions list                  # list versions

# Devices & Provisioning
apple-cli devices list                   # list devices
apple-cli certificates list              # list certificates
apple-cli profiles list                  # list provisioning profiles

# Users
apple-cli users list                     # list team users
```

## Project Config

The `.apple/config.toml` in this project directory sets:
- `profile` — which Apple Developer account to use
- `app_id` — default app ID for commands that filter by app
- `app_name` — human-readable app name
- `bundle_id` — app bundle identifier

To reconfigure: `apple-cli init`
"#);

        std::fs::write(skill_dir.join("apple.md"), skill_content)?;
        Ok(())
    }
}
