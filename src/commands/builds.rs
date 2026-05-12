use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::build::BuildAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum BuildsCommands {
    /// List builds
    List {
        #[arg(long)]
        filter_app: Option<String>,
        #[arg(long)]
        filter_version: Option<String>,
        #[arg(long)]
        filter_processing_state: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Get a single build by ID
    Get { id: String },
}

pub async fn execute(
    cmd: &BuildsCommands,
    client: &ApiClient,
    project_app_id: Option<&str>,
) -> Result<(), CliError> {
    match cmd {
        BuildsCommands::List {
            filter_app,
            filter_version,
            filter_processing_state,
            limit,
            all,
        } => {
            let app_id = filter_app.as_deref().or(project_app_id);
            let mut params = vec![format!("limit={limit}")];
            if let Some(v) = app_id {
                params.push(format!("filter[app]={v}"));
            }
            if let Some(v) = filter_version {
                params.push(format!("filter[version]={v}"));
            }
            if let Some(v) = filter_processing_state {
                params.push(format!("filter[processingState]={v}"));
            }
            let path = format!("/builds?{}", params.join("&"));

            let doc = if *all {
                client.get_all::<BuildAttributes>(&path).await?
            } else {
                client.get_list::<BuildAttributes>(&path).await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BuildsCommands::Get { id } => {
            let doc = client
                .get::<BuildAttributes>(&format!("/builds/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
