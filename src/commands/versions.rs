use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::app_store_version::AppStoreVersionAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum VersionsCommands {
    /// List app store versions
    List {
        #[arg(long)]
        filter_app: Option<String>,
        #[arg(long)]
        filter_platform: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Get a single version by ID
    Get { id: String },
    /// Create a new app store version
    Create {
        /// App ID (falls back to project config)
        #[arg(long)]
        app_id: Option<String>,
        /// Platform: IOS, MAC_OS, TV_OS, VISION_OS
        #[arg(long)]
        platform: String,
        /// Version string (e.g. "1.2.0")
        #[arg(long)]
        version_string: String,
        /// Release type: MANUAL, AFTER_APPROVAL, SCHEDULED (defaults to AFTER_APPROVAL)
        #[arg(long)]
        release_type: Option<String>,
    },
    /// Update an existing app store version
    Update {
        /// Version ID
        id: String,
        /// New version string
        #[arg(long)]
        version_string: Option<String>,
        /// Copyright text
        #[arg(long)]
        copyright: Option<String>,
        /// Release type: MANUAL, AFTER_APPROVAL, SCHEDULED
        #[arg(long)]
        release_type: Option<String>,
        /// Earliest release date (ISO 8601)
        #[arg(long)]
        earliest_release_date: Option<String>,
    },
    /// Delete an app store version
    Delete {
        /// Version ID
        id: String,
    },
    /// Attach a build to a version
    #[command(name = "attach-build")]
    AttachBuild {
        /// Version ID
        #[arg(long)]
        version_id: String,
        /// Build ID
        #[arg(long)]
        build_id: String,
    },
}

pub async fn execute(
    cmd: &VersionsCommands,
    client: &ApiClient,
    project_app_id: Option<&str>,
) -> Result<(), CliError> {
    match cmd {
        VersionsCommands::List {
            filter_app,
            filter_platform,
            limit,
            all,
        } => {
            let app_id = filter_app.as_deref().or(project_app_id).ok_or_else(|| {
                CliError::Config(
                    "app ID required for versions list (use --filter-app or run `apple-cli init` in your project)".into(),
                )
            })?;

            let mut params = vec![format!("limit={limit}")];
            if let Some(v) = filter_platform {
                params.push(format!("filter[platform]={v}"));
            }
            let path = format!("/apps/{app_id}/appStoreVersions?{}", params.join("&"));

            let doc = if *all {
                client.get_all::<AppStoreVersionAttributes>(&path).await?
            } else {
                client.get_list::<AppStoreVersionAttributes>(&path).await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        VersionsCommands::Get { id } => {
            let doc = client
                .get::<AppStoreVersionAttributes>(&format!("/appStoreVersions/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        VersionsCommands::Create {
            app_id,
            platform,
            version_string,
            release_type,
        } => {
            let resolved_app_id = app_id.as_deref().or(project_app_id).ok_or_else(|| {
                CliError::Config(
                    "app ID required (use --app-id or run `apple-cli init` in your project)"
                        .into(),
                )
            })?;

            let mut attrs = serde_json::Map::new();
            attrs.insert("platform".into(), serde_json::json!(platform));
            attrs.insert("versionString".into(), serde_json::json!(version_string));
            attrs.insert(
                "releaseType".into(),
                serde_json::json!(release_type.as_deref().unwrap_or("AFTER_APPROVAL")),
            );

            let body = serde_json::json!({
                "data": {
                    "type": "appStoreVersions",
                    "attributes": attrs,
                    "relationships": {
                        "app": {
                            "data": {
                                "id": resolved_app_id,
                                "type": "apps"
                            }
                        }
                    }
                }
            });

            let doc = client
                .post::<_, AppStoreVersionAttributes>("/appStoreVersions", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        VersionsCommands::Update {
            id,
            version_string,
            copyright,
            release_type,
            earliest_release_date,
        } => {
            let mut attrs = serde_json::Map::new();
            if let Some(v) = version_string {
                attrs.insert("versionString".into(), serde_json::json!(v));
            }
            if let Some(v) = copyright {
                attrs.insert("copyright".into(), serde_json::json!(v));
            }
            if let Some(v) = release_type {
                attrs.insert("releaseType".into(), serde_json::json!(v));
            }
            if let Some(v) = earliest_release_date {
                attrs.insert("earliestReleaseDate".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "type": "appStoreVersions",
                    "id": id,
                    "attributes": attrs
                }
            });

            let doc = client
                .patch::<_, AppStoreVersionAttributes>(
                    &format!("/appStoreVersions/{id}"),
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        VersionsCommands::Delete { id } => {
            client
                .delete(&format!("/appStoreVersions/{id}"))
                .await?;
            let output =
                serde_json::json!({ "message": format!("App Store version {id} deleted") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        VersionsCommands::AttachBuild {
            version_id,
            build_id,
        } => {
            let body = serde_json::json!({
                "data": {
                    "type": "builds",
                    "id": build_id
                }
            });

            let doc = client
                .patch::<_, serde_json::Value>(
                    &format!("/appStoreVersions/{version_id}/relationships/build"),
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
