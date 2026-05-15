use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::territory::TerritoryAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum TerritoriesCommands {
    /// List all territories
    List {
        #[arg(long, default_value = "200")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
}

pub async fn execute(
    cmd: &TerritoriesCommands,
    client: &ApiClient,
) -> Result<(), CliError> {
    match cmd {
        TerritoriesCommands::List { limit, all } => {
            let path = format!("/territories?limit={limit}");

            let doc = if *all {
                client.get_all::<TerritoryAttributes>(&path).await?
            } else {
                client.get_list::<TerritoryAttributes>(&path).await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
