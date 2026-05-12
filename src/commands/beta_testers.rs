use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::beta_tester::BetaTesterAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum BetaTestersCommands {
    /// List beta testers
    List {
        #[arg(long)]
        filter_email: Option<String>,
        #[arg(long)]
        filter_app: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Get a single beta tester by ID
    Get { id: String },
    /// Add a beta tester
    Add {
        #[arg(long)]
        email: String,
        #[arg(long)]
        first_name: String,
        #[arg(long)]
        last_name: String,
        #[arg(long, value_delimiter = ',')]
        beta_group_ids: Vec<String>,
    },
    /// Remove a beta tester
    Remove { id: String },
}

pub async fn execute(cmd: &BetaTestersCommands, client: &ApiClient) -> Result<(), CliError> {
    match cmd {
        BetaTestersCommands::List {
            filter_email,
            filter_app,
            limit,
            all,
        } => {
            let mut params = vec![format!("limit={limit}")];
            if let Some(v) = filter_email {
                params.push(format!("filter[email]={v}"));
            }
            if let Some(v) = filter_app {
                params.push(format!("filter[apps]={v}"));
            }
            let path = format!("/betaTesters?{}", params.join("&"));

            let doc = if *all {
                client.get_all::<BetaTesterAttributes>(&path).await?
            } else {
                client.get_list::<BetaTesterAttributes>(&path).await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BetaTestersCommands::Get { id } => {
            let doc = client
                .get::<BetaTesterAttributes>(&format!("/betaTesters/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BetaTestersCommands::Add {
            email,
            first_name,
            last_name,
            beta_group_ids,
        } => {
            let group_data: Vec<serde_json::Value> = beta_group_ids
                .iter()
                .map(|id| {
                    serde_json::json!({
                        "id": id,
                        "type": "betaGroups"
                    })
                })
                .collect();

            let body = serde_json::json!({
                "data": {
                    "type": "betaTesters",
                    "attributes": {
                        "email": email,
                        "firstName": first_name,
                        "lastName": last_name,
                    },
                    "relationships": {
                        "betaGroups": {
                            "data": group_data
                        }
                    }
                }
            });

            let doc = client
                .post::<_, BetaTesterAttributes>("/betaTesters", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BetaTestersCommands::Remove { id } => {
            client.delete(&format!("/betaTesters/{id}")).await?;
            let output = serde_json::json!({ "message": format!("Beta tester {id} removed") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
