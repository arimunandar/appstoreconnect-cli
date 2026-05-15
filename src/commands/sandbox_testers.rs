use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::sandbox_tester::SandboxTesterAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SandboxTestersCommands {
    /// List sandbox testers (v2 endpoint)
    List,
    /// Update a sandbox tester (v2 endpoint)
    Update {
        /// Sandbox tester ID
        id: String,
        /// Territory (e.g. USA, JPN)
        #[arg(long)]
        territory: Option<String>,
        /// Subscription renewal rate (e.g. MONTHLY_RENEWAL_EVERY_THIRTY_SECONDS)
        #[arg(long)]
        renewal_rate: Option<String>,
        /// Whether purchases can be interrupted
        #[arg(long)]
        interrupted_purchases: Option<bool>,
    },
    /// Clear purchase history for a sandbox tester (v2 endpoint)
    #[command(name = "clear-history")]
    ClearPurchaseHistory {
        /// Sandbox tester ID
        #[arg(long)]
        tester_id: String,
    },
}

pub async fn execute(
    cmd: &SandboxTestersCommands,
    client: &ApiClient,
) -> Result<(), CliError> {
    match cmd {
        SandboxTestersCommands::List => {
            let path = "/../v2/sandboxTesters";
            let doc = client.get_all::<SandboxTesterAttributes>(path).await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        SandboxTestersCommands::Update {
            id,
            territory,
            renewal_rate,
            interrupted_purchases,
        } => {
            let mut attrs = serde_json::Map::new();
            if let Some(v) = territory {
                attrs.insert("territory".into(), serde_json::json!(v));
            }
            if let Some(v) = renewal_rate {
                attrs.insert(
                    "subscriptionRenewalRate".into(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = interrupted_purchases {
                attrs.insert("interruptedPurchases".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "type": "sandboxTesters",
                    "id": id,
                    "attributes": attrs
                }
            });

            let doc = client
                .patch::<_, SandboxTesterAttributes>(
                    &format!("/../v2/sandboxTesters/{id}"),
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        SandboxTestersCommands::ClearPurchaseHistory { tester_id } => {
            let body = serde_json::json!({
                "data": {
                    "type": "sandboxTestersClearPurchaseHistoryRequest",
                    "relationships": {
                        "sandboxTesters": {
                            "data": [{
                                "id": tester_id,
                                "type": "sandboxTesters"
                            }]
                        }
                    }
                }
            });

            let doc = client
                .post::<_, serde_json::Value>(
                    "/../v2/sandboxTestersClearPurchaseHistoryRequest",
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
