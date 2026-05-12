use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::bundle_id::BundleIdAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum BundleIdsCommands {
    /// List bundle IDs
    List {
        #[arg(long)]
        filter_identifier: Option<String>,
        #[arg(long)]
        filter_platform: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Get a single bundle ID by ID
    Get { id: String },
    /// Register a new bundle ID
    Create {
        #[arg(long)]
        identifier: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        platform: String,
    },
    /// Delete a bundle ID
    Delete { id: String },
}

pub async fn execute(cmd: &BundleIdsCommands, client: &ApiClient) -> Result<(), CliError> {
    match cmd {
        BundleIdsCommands::List {
            filter_identifier,
            filter_platform,
            limit,
            all,
        } => {
            let mut params = vec![format!("limit={limit}")];
            if let Some(v) = filter_identifier {
                params.push(format!("filter[identifier]={v}"));
            }
            if let Some(v) = filter_platform {
                params.push(format!("filter[platform]={v}"));
            }
            let path = format!("/bundleIds?{}", params.join("&"));

            let doc = if *all {
                client.get_all::<BundleIdAttributes>(&path).await?
            } else {
                client.get_list::<BundleIdAttributes>(&path).await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BundleIdsCommands::Get { id } => {
            let doc = client
                .get::<BundleIdAttributes>(&format!("/bundleIds/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BundleIdsCommands::Create {
            identifier,
            name,
            platform,
        } => {
            let body = serde_json::json!({
                "data": {
                    "type": "bundleIds",
                    "attributes": {
                        "identifier": identifier,
                        "name": name,
                        "platform": platform,
                    }
                }
            });

            let doc = client
                .post::<_, BundleIdAttributes>("/bundleIds", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BundleIdsCommands::Delete { id } => {
            client.delete(&format!("/bundleIds/{id}")).await?;
            let output = serde_json::json!({ "message": format!("Bundle ID {id} deleted") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
