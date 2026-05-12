use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::device::DeviceAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum DevicesCommands {
    /// List registered devices
    List {
        #[arg(long)]
        filter_name: Option<String>,
        #[arg(long)]
        filter_platform: Option<String>,
        #[arg(long)]
        filter_udid: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Get a single device by ID
    Get { id: String },
    /// Register a new device
    Register {
        #[arg(long)]
        name: String,
        #[arg(long)]
        platform: String,
        #[arg(long)]
        udid: String,
    },
    /// Update a device (e.g. rename or change status)
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },
}

pub async fn execute(cmd: &DevicesCommands, client: &ApiClient) -> Result<(), CliError> {
    match cmd {
        DevicesCommands::List {
            filter_name,
            filter_platform,
            filter_udid,
            limit,
            all,
        } => {
            let mut params = vec![format!("limit={limit}")];
            if let Some(v) = filter_name {
                params.push(format!("filter[name]={v}"));
            }
            if let Some(v) = filter_platform {
                params.push(format!("filter[platform]={v}"));
            }
            if let Some(v) = filter_udid {
                params.push(format!("filter[udid]={v}"));
            }
            let path = format!("/devices?{}", params.join("&"));

            let doc = if *all {
                client.get_all::<DeviceAttributes>(&path).await?
            } else {
                client.get_list::<DeviceAttributes>(&path).await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        DevicesCommands::Get { id } => {
            let doc = client
                .get::<DeviceAttributes>(&format!("/devices/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        DevicesCommands::Register {
            name,
            platform,
            udid,
        } => {
            let body = serde_json::json!({
                "data": {
                    "type": "devices",
                    "attributes": {
                        "name": name,
                        "platform": platform,
                        "udid": udid,
                    }
                }
            });

            let doc = client
                .post::<_, DeviceAttributes>("/devices", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        DevicesCommands::Update { id, name, status } => {
            let mut attrs = serde_json::Map::new();
            if let Some(v) = name {
                attrs.insert("name".into(), serde_json::json!(v));
            }
            if let Some(v) = status {
                attrs.insert("status".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "id": id,
                    "type": "devices",
                    "attributes": attrs,
                }
            });

            let doc = client
                .patch::<_, DeviceAttributes>(&format!("/devices/{id}"), &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
