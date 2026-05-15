use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::bundle_id_capability::BundleIdCapabilityAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum BundleIdCapabilitiesCommands {
    /// List capabilities for a bundle ID
    List {
        /// Bundle ID resource ID
        #[arg(long)]
        bundle_id: String,
    },
    /// Enable a capability for a bundle ID
    Enable {
        /// Bundle ID resource ID
        #[arg(long)]
        bundle_id: String,
        /// Capability type (e.g. ICLOUD, PUSH_NOTIFICATIONS, GAME_CENTER)
        #[arg(long)]
        capability_type: String,
    },
    /// Disable (delete) a capability
    Disable {
        /// Bundle ID capability resource ID
        id: String,
    },
}

pub async fn execute(
    cmd: &BundleIdCapabilitiesCommands,
    client: &ApiClient,
) -> Result<(), CliError> {
    match cmd {
        BundleIdCapabilitiesCommands::List { bundle_id } => {
            let path = format!("/bundleIds/{bundle_id}/bundleIdCapabilities");
            let doc = client
                .get_all::<BundleIdCapabilityAttributes>(&path)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BundleIdCapabilitiesCommands::Enable {
            bundle_id,
            capability_type,
        } => {
            let body = serde_json::json!({
                "data": {
                    "type": "bundleIdCapabilities",
                    "attributes": {
                        "capabilityType": capability_type
                    },
                    "relationships": {
                        "bundleId": {
                            "data": {
                                "id": bundle_id,
                                "type": "bundleIds"
                            }
                        }
                    }
                }
            });

            let doc = client
                .post::<_, BundleIdCapabilityAttributes>("/bundleIdCapabilities", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BundleIdCapabilitiesCommands::Disable { id } => {
            client
                .delete(&format!("/bundleIdCapabilities/{id}"))
                .await?;
            let output =
                serde_json::json!({ "message": format!("Capability {id} disabled") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
