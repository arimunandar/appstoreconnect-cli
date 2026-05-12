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
            let app_id = filter_app.as_deref().or(project_app_id);
            let mut params = vec![format!("limit={limit}")];
            if let Some(v) = app_id {
                params.push(format!("filter[app]={v}"));
            }
            if let Some(v) = filter_platform {
                params.push(format!("filter[platform]={v}"));
            }
            let path = format!("/appStoreVersions?{}", params.join("&"));

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
    }
}
