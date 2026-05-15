use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::app_info_localization::AppInfoLocalizationAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum AppInfoLocalizationsCommands {
    /// List app info localizations for an app info
    List {
        /// App Info ID
        #[arg(long)]
        app_info_id: String,
    },
    /// Get an app info localization by ID
    Get {
        /// App info localization ID
        #[arg(long)]
        id: String,
    },
    /// Create an app info localization
    Create {
        /// App Info ID (relationship)
        #[arg(long)]
        app_info_id: String,
        /// Locale code (e.g. en-US, id, ja)
        #[arg(long)]
        locale: String,
        /// App name
        #[arg(long)]
        name: Option<String>,
        /// App subtitle
        #[arg(long)]
        subtitle: Option<String>,
        /// Privacy policy URL
        #[arg(long)]
        privacy_policy_url: Option<String>,
        /// Privacy policy text
        #[arg(long)]
        privacy_policy_text: Option<String>,
        /// Privacy choices URL
        #[arg(long)]
        privacy_choices_url: Option<String>,
    },
    /// Update an app info localization
    Update {
        /// App info localization ID
        #[arg(long)]
        id: String,
        /// App name
        #[arg(long)]
        name: Option<String>,
        /// App subtitle
        #[arg(long)]
        subtitle: Option<String>,
        /// Privacy policy URL
        #[arg(long)]
        privacy_policy_url: Option<String>,
        /// Privacy policy text
        #[arg(long)]
        privacy_policy_text: Option<String>,
        /// Privacy choices URL
        #[arg(long)]
        privacy_choices_url: Option<String>,
    },
    /// Delete an app info localization
    Delete {
        /// App info localization ID
        #[arg(long)]
        id: String,
    },
}

pub async fn execute(
    cmd: &AppInfoLocalizationsCommands,
    client: &ApiClient,
) -> Result<(), CliError> {
    match cmd {
        AppInfoLocalizationsCommands::List { app_info_id } => {
            let doc = client
                .get_all::<AppInfoLocalizationAttributes>(&format!(
                    "/appInfos/{app_info_id}/appInfoLocalizations"
                ))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        AppInfoLocalizationsCommands::Get { id } => {
            let doc = client
                .get::<AppInfoLocalizationAttributes>(&format!("/appInfoLocalizations/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        AppInfoLocalizationsCommands::Create {
            app_info_id,
            locale,
            name,
            subtitle,
            privacy_policy_url,
            privacy_policy_text,
            privacy_choices_url,
        } => {
            let mut attrs = serde_json::Map::new();
            attrs.insert("locale".into(), serde_json::json!(locale));
            if let Some(v) = name {
                attrs.insert("name".into(), serde_json::json!(v));
            }
            if let Some(v) = subtitle {
                attrs.insert("subtitle".into(), serde_json::json!(v));
            }
            if let Some(v) = privacy_policy_url {
                attrs.insert("privacyPolicyUrl".into(), serde_json::json!(v));
            }
            if let Some(v) = privacy_policy_text {
                attrs.insert("privacyPolicyText".into(), serde_json::json!(v));
            }
            if let Some(v) = privacy_choices_url {
                attrs.insert("privacyChoicesUrl".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "type": "appInfoLocalizations",
                    "attributes": attrs,
                    "relationships": {
                        "appInfo": {
                            "data": {
                                "id": app_info_id,
                                "type": "appInfos"
                            }
                        }
                    }
                }
            });

            let doc = client
                .post::<_, AppInfoLocalizationAttributes>("/appInfoLocalizations", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        AppInfoLocalizationsCommands::Update {
            id,
            name,
            subtitle,
            privacy_policy_url,
            privacy_policy_text,
            privacy_choices_url,
        } => {
            let mut attrs = serde_json::Map::new();
            if let Some(v) = name {
                attrs.insert("name".into(), serde_json::json!(v));
            }
            if let Some(v) = subtitle {
                attrs.insert("subtitle".into(), serde_json::json!(v));
            }
            if let Some(v) = privacy_policy_url {
                attrs.insert("privacyPolicyUrl".into(), serde_json::json!(v));
            }
            if let Some(v) = privacy_policy_text {
                attrs.insert("privacyPolicyText".into(), serde_json::json!(v));
            }
            if let Some(v) = privacy_choices_url {
                attrs.insert("privacyChoicesUrl".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "type": "appInfoLocalizations",
                    "id": id,
                    "attributes": attrs
                }
            });

            let doc = client
                .patch::<_, AppInfoLocalizationAttributes>(
                    &format!("/appInfoLocalizations/{id}"),
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        AppInfoLocalizationsCommands::Delete { id } => {
            client
                .delete(&format!("/appInfoLocalizations/{id}"))
                .await?;
            let output =
                serde_json::json!({ "message": format!("App info localization {id} deleted") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
