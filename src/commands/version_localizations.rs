use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::version_localization::AppStoreVersionLocalizationAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum VersionLocalizationsCommands {
    /// List localizations for a version
    List {
        /// App Store Version ID
        #[arg(long)]
        version_id: String,
    },
    /// Get a single version localization by ID
    Get { id: String },
    /// Create a localization for a version
    Create {
        /// App Store Version ID
        #[arg(long)]
        version_id: String,
        /// Locale code (e.g. en-US, id, ja)
        #[arg(long)]
        locale: String,
        /// App description
        #[arg(long)]
        description: Option<String>,
        /// Search keywords
        #[arg(long)]
        keywords: Option<String>,
        /// Marketing URL
        #[arg(long)]
        marketing_url: Option<String>,
        /// Promotional text
        #[arg(long)]
        promotional_text: Option<String>,
        /// Support URL
        #[arg(long)]
        support_url: Option<String>,
        /// What's new text (release notes)
        #[arg(long)]
        whats_new: Option<String>,
    },
    /// Update a version localization
    Update {
        /// Version localization ID
        id: String,
        /// App description
        #[arg(long)]
        description: Option<String>,
        /// Search keywords
        #[arg(long)]
        keywords: Option<String>,
        /// Marketing URL
        #[arg(long)]
        marketing_url: Option<String>,
        /// Promotional text
        #[arg(long)]
        promotional_text: Option<String>,
        /// Support URL
        #[arg(long)]
        support_url: Option<String>,
        /// What's new text (release notes)
        #[arg(long)]
        whats_new: Option<String>,
    },
    /// Delete a version localization
    Delete {
        /// Version localization ID
        id: String,
    },
}

pub async fn execute(
    cmd: &VersionLocalizationsCommands,
    client: &ApiClient,
) -> Result<(), CliError> {
    match cmd {
        VersionLocalizationsCommands::List { version_id } => {
            let path = format!(
                "/appStoreVersions/{version_id}/appStoreVersionLocalizations"
            );
            let doc = client
                .get_list::<AppStoreVersionLocalizationAttributes>(&path)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        VersionLocalizationsCommands::Get { id } => {
            let doc = client
                .get::<AppStoreVersionLocalizationAttributes>(
                    &format!("/appStoreVersionLocalizations/{id}"),
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        VersionLocalizationsCommands::Create {
            version_id,
            locale,
            description,
            keywords,
            marketing_url,
            promotional_text,
            support_url,
            whats_new,
        } => {
            let mut attrs = serde_json::Map::new();
            attrs.insert("locale".into(), serde_json::json!(locale));

            if let Some(v) = description {
                attrs.insert("description".into(), serde_json::json!(v));
            }
            if let Some(v) = keywords {
                attrs.insert("keywords".into(), serde_json::json!(v));
            }
            if let Some(v) = marketing_url {
                attrs.insert("marketingUrl".into(), serde_json::json!(v));
            }
            if let Some(v) = promotional_text {
                attrs.insert("promotionalText".into(), serde_json::json!(v));
            }
            if let Some(v) = support_url {
                attrs.insert("supportUrl".into(), serde_json::json!(v));
            }
            if let Some(v) = whats_new {
                attrs.insert("whatsNew".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "type": "appStoreVersionLocalizations",
                    "attributes": attrs,
                    "relationships": {
                        "appStoreVersion": {
                            "data": {
                                "id": version_id,
                                "type": "appStoreVersions"
                            }
                        }
                    }
                }
            });

            let doc = client
                .post::<_, AppStoreVersionLocalizationAttributes>(
                    "/appStoreVersionLocalizations",
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        VersionLocalizationsCommands::Update {
            id,
            description,
            keywords,
            marketing_url,
            promotional_text,
            support_url,
            whats_new,
        } => {
            let mut attrs = serde_json::Map::new();
            if let Some(v) = description {
                attrs.insert("description".into(), serde_json::json!(v));
            }
            if let Some(v) = keywords {
                attrs.insert("keywords".into(), serde_json::json!(v));
            }
            if let Some(v) = marketing_url {
                attrs.insert("marketingUrl".into(), serde_json::json!(v));
            }
            if let Some(v) = promotional_text {
                attrs.insert("promotionalText".into(), serde_json::json!(v));
            }
            if let Some(v) = support_url {
                attrs.insert("supportUrl".into(), serde_json::json!(v));
            }
            if let Some(v) = whats_new {
                attrs.insert("whatsNew".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "type": "appStoreVersionLocalizations",
                    "id": id,
                    "attributes": attrs
                }
            });

            let doc = client
                .patch::<_, AppStoreVersionLocalizationAttributes>(
                    &format!("/appStoreVersionLocalizations/{id}"),
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        VersionLocalizationsCommands::Delete { id } => {
            client
                .delete(&format!("/appStoreVersionLocalizations/{id}"))
                .await?;
            let output = serde_json::json!({ "message": format!("Version localization {id} deleted") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
