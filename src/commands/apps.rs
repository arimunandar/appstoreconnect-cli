use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::app::AppAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum AppsCommands {
    /// List apps
    List {
        #[arg(long)]
        filter_bundle_id: Option<String>,
        #[arg(long)]
        filter_name: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
        #[arg(long)]
        sort: Option<String>,
    },
    /// Get a single app by ID
    Get {
        /// App resource ID
        id: String,
    },
}

pub async fn execute(cmd: &AppsCommands, client: &ApiClient) -> Result<(), CliError> {
    match cmd {
        AppsCommands::List {
            filter_bundle_id,
            filter_name,
            limit,
            all,
            sort,
        } => {
            let mut params = vec![format!("limit={limit}")];
            if let Some(v) = filter_bundle_id {
                params.push(format!("filter[bundleId]={v}"));
            }
            if let Some(v) = filter_name {
                params.push(format!("filter[name]={v}"));
            }
            if let Some(v) = sort {
                params.push(format!("sort={v}"));
            }
            let path = format!("/apps?{}", params.join("&"));

            let doc = if *all {
                client.get_all::<AppAttributes>(&path).await?
            } else {
                client.get_list::<AppAttributes>(&path).await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        AppsCommands::Get { id } => {
            let doc = client.get::<AppAttributes>(&format!("/apps/{id}")).await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
