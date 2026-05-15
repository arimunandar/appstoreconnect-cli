use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::phased_release::AppStoreVersionPhasedReleaseAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum PhasedReleasesCommands {
    /// Create a phased release for a version
    Create {
        /// App Store Version ID
        #[arg(long)]
        version_id: String,
    },
    /// Pause a phased release
    Pause {
        /// Phased release ID
        id: String,
    },
    /// Resume a paused phased release
    Resume {
        /// Phased release ID
        id: String,
    },
    /// Complete a phased release immediately
    Complete {
        /// Phased release ID
        id: String,
    },
    /// Delete a phased release
    Delete {
        /// Phased release ID
        id: String,
    },
}

pub async fn execute(
    cmd: &PhasedReleasesCommands,
    client: &ApiClient,
) -> Result<(), CliError> {
    match cmd {
        PhasedReleasesCommands::Create { version_id } => {
            let body = serde_json::json!({
                "data": {
                    "type": "appStoreVersionPhasedReleases",
                    "attributes": {
                        "phasedReleaseState": "ACTIVE"
                    },
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
                .post::<_, AppStoreVersionPhasedReleaseAttributes>(
                    "/appStoreVersionPhasedReleases",
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        PhasedReleasesCommands::Pause { id } => {
            let body = serde_json::json!({
                "data": {
                    "type": "appStoreVersionPhasedReleases",
                    "id": id,
                    "attributes": {
                        "phasedReleaseState": "PAUSED"
                    }
                }
            });

            let doc = client
                .patch::<_, AppStoreVersionPhasedReleaseAttributes>(
                    &format!("/appStoreVersionPhasedReleases/{id}"),
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        PhasedReleasesCommands::Resume { id } => {
            let body = serde_json::json!({
                "data": {
                    "type": "appStoreVersionPhasedReleases",
                    "id": id,
                    "attributes": {
                        "phasedReleaseState": "ACTIVE"
                    }
                }
            });

            let doc = client
                .patch::<_, AppStoreVersionPhasedReleaseAttributes>(
                    &format!("/appStoreVersionPhasedReleases/{id}"),
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        PhasedReleasesCommands::Complete { id } => {
            let body = serde_json::json!({
                "data": {
                    "type": "appStoreVersionPhasedReleases",
                    "id": id,
                    "attributes": {
                        "phasedReleaseState": "COMPLETE"
                    }
                }
            });

            let doc = client
                .patch::<_, AppStoreVersionPhasedReleaseAttributes>(
                    &format!("/appStoreVersionPhasedReleases/{id}"),
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        PhasedReleasesCommands::Delete { id } => {
            client
                .delete(&format!("/appStoreVersionPhasedReleases/{id}"))
                .await?;
            let output = serde_json::json!({ "message": format!("Phased release {id} deleted") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
