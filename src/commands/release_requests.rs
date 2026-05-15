use crate::client::ApiClient;
use crate::error::CliError;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ReleaseRequestsCommands {
    /// Create a release request to manually release an approved version
    Create {
        /// App Store Version ID (must be in "Pending Developer Release" state)
        #[arg(long)]
        version_id: String,
    },
}

pub async fn execute(
    cmd: &ReleaseRequestsCommands,
    client: &ApiClient,
) -> Result<(), CliError> {
    match cmd {
        ReleaseRequestsCommands::Create { version_id } => {
            let body = serde_json::json!({
                "data": {
                    "type": "appStoreVersionReleaseRequests",
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
                .post::<_, serde_json::Value>("/appStoreVersionReleaseRequests", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
