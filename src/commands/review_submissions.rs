use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::review_submission::{ReviewSubmissionAttributes, ReviewSubmissionItemAttributes};
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ReviewSubmissionsCommands {
    /// List review submissions for the app
    List {
        #[arg(long)]
        app_id: Option<String>,
    },
    /// Get a review submission by ID
    Get { id: String },
    /// Create a review submission for the app
    Create {
        #[arg(long)]
        app_id: Option<String>,
        /// Platform: IOS, MAC_OS, TV_OS, VISION_OS
        #[arg(long)]
        platform: String,
    },
    /// Submit a review submission for review
    Submit { id: String },
    /// Cancel a review submission
    Cancel { id: String },
    /// List items in a review submission
    Items { id: String },
    /// Add an item to a review submission
    #[command(name = "add-item")]
    AddItem {
        /// Review submission ID
        #[arg(long)]
        submission_id: String,
        /// App Store version ID to include
        #[arg(long)]
        app_store_version_id: Option<String>,
    },
}

pub async fn execute(
    cmd: &ReviewSubmissionsCommands,
    client: &ApiClient,
    project_app_id: Option<&str>,
) -> Result<(), CliError> {
    match cmd {
        ReviewSubmissionsCommands::List { app_id } => {
            let resolved_app_id = app_id.as_deref().or(project_app_id).ok_or_else(|| {
                CliError::Config(
                    "app-id required (use --app-id or run `apple-cli init` in your project)".into(),
                )
            })?;

            let path = format!("/reviewSubmissions?filter[app]={resolved_app_id}");
            let doc = client.get_all::<ReviewSubmissionAttributes>(&path).await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        ReviewSubmissionsCommands::Get { id } => {
            let doc = client
                .get::<ReviewSubmissionAttributes>(&format!("/reviewSubmissions/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        ReviewSubmissionsCommands::Create { app_id, platform } => {
            let resolved_app_id = app_id.as_deref().or(project_app_id).ok_or_else(|| {
                CliError::Config(
                    "app-id required (use --app-id or run `apple-cli init` in your project)".into(),
                )
            })?;

            let body = serde_json::json!({
                "data": {
                    "type": "reviewSubmissions",
                    "attributes": {
                        "platform": platform,
                    },
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
                .post::<_, ReviewSubmissionAttributes>("/reviewSubmissions", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        ReviewSubmissionsCommands::Submit { id } => {
            let body = serde_json::json!({
                "data": {
                    "type": "reviewSubmissions",
                    "id": id,
                    "attributes": {
                        "submitted": true,
                    }
                }
            });

            let doc = client
                .patch::<_, ReviewSubmissionAttributes>(&format!("/reviewSubmissions/{id}"), &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        ReviewSubmissionsCommands::Cancel { id } => {
            let body = serde_json::json!({
                "data": {
                    "type": "reviewSubmissions",
                    "id": id,
                    "attributes": {
                        "canceled": true,
                    }
                }
            });

            let doc = client
                .patch::<_, ReviewSubmissionAttributes>(&format!("/reviewSubmissions/{id}"), &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        ReviewSubmissionsCommands::Items { id } => {
            let path = format!("/reviewSubmissions/{id}/items");
            let doc = client
                .get_all::<ReviewSubmissionItemAttributes>(&path)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        ReviewSubmissionsCommands::AddItem {
            submission_id,
            app_store_version_id,
        } => {
            let mut relationships = serde_json::Map::new();
            relationships.insert(
                "reviewSubmission".into(),
                serde_json::json!({
                    "data": {
                        "id": submission_id,
                        "type": "reviewSubmissions"
                    }
                }),
            );

            if let Some(version_id) = app_store_version_id {
                relationships.insert(
                    "appStoreVersion".into(),
                    serde_json::json!({
                        "data": {
                            "id": version_id,
                            "type": "appStoreVersions"
                        }
                    }),
                );
            }

            let body = serde_json::json!({
                "data": {
                    "type": "reviewSubmissionItems",
                    "relationships": relationships,
                }
            });

            let doc = client
                .post::<_, ReviewSubmissionItemAttributes>("/reviewSubmissionItems", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
