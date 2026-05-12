use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::profile::ProfileAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ProfilesCommands {
    /// List provisioning profiles
    List {
        #[arg(long)]
        filter_name: Option<String>,
        #[arg(long)]
        filter_type: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Get a single profile by ID
    Get { id: String },
    /// Create a provisioning profile
    Create {
        #[arg(long)]
        name: String,
        /// Profile type (e.g., IOS_APP_DEVELOPMENT, IOS_APP_STORE)
        #[arg(long, rename_all = "verbatim")]
        r#type: String,
        /// Bundle ID resource ID
        #[arg(long)]
        bundle_id: String,
        /// Certificate resource IDs (comma-separated)
        #[arg(long, value_delimiter = ',')]
        certificate_ids: Vec<String>,
        /// Device resource IDs (comma-separated, required for development profiles)
        #[arg(long, value_delimiter = ',')]
        device_ids: Option<Vec<String>>,
    },
    /// Delete a provisioning profile
    Delete { id: String },
}

pub async fn execute(cmd: &ProfilesCommands, client: &ApiClient) -> Result<(), CliError> {
    match cmd {
        ProfilesCommands::List {
            filter_name,
            filter_type,
            limit,
            all,
        } => {
            let mut params = vec![format!("limit={limit}")];
            if let Some(v) = filter_name {
                params.push(format!("filter[name]={v}"));
            }
            if let Some(v) = filter_type {
                params.push(format!("filter[profileType]={v}"));
            }
            let path = format!("/profiles?{}", params.join("&"));

            let doc = if *all {
                client.get_all::<ProfileAttributes>(&path).await?
            } else {
                client.get_list::<ProfileAttributes>(&path).await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        ProfilesCommands::Get { id } => {
            let doc = client
                .get::<ProfileAttributes>(&format!("/profiles/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        ProfilesCommands::Create {
            name,
            r#type,
            bundle_id,
            certificate_ids,
            device_ids,
        } => {
            let cert_data: Vec<serde_json::Value> = certificate_ids
                .iter()
                .map(|id| serde_json::json!({"id": id, "type": "certificates"}))
                .collect();

            let mut relationships = serde_json::json!({
                "bundleId": {
                    "data": {
                        "id": bundle_id,
                        "type": "bundleIds"
                    }
                },
                "certificates": {
                    "data": cert_data
                }
            });

            if let Some(dev_ids) = device_ids {
                let dev_data: Vec<serde_json::Value> = dev_ids
                    .iter()
                    .map(|id| serde_json::json!({"id": id, "type": "devices"}))
                    .collect();
                relationships["devices"] = serde_json::json!({ "data": dev_data });
            }

            let body = serde_json::json!({
                "data": {
                    "type": "profiles",
                    "attributes": {
                        "name": name,
                        "profileType": r#type,
                    },
                    "relationships": relationships,
                }
            });

            let doc = client
                .post::<_, ProfileAttributes>("/profiles", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        ProfilesCommands::Delete { id } => {
            client.delete(&format!("/profiles/{id}")).await?;
            let output = serde_json::json!({ "message": format!("Profile {id} deleted") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
