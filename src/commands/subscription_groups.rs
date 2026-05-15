use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::subscription::SubscriptionGroupAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SubscriptionGroupsCommands {
    /// List subscription groups for the app
    List {
        #[arg(long)]
        app_id: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Create a subscription group
    Create {
        #[arg(long)]
        app_id: Option<String>,
        #[arg(long)]
        name: String,
    },
    /// Delete a subscription group
    Delete { id: String },
}

pub async fn execute(
    cmd: &SubscriptionGroupsCommands,
    client: &ApiClient,
    project_app_id: Option<&str>,
) -> Result<(), CliError> {
    match cmd {
        SubscriptionGroupsCommands::List {
            app_id,
            limit,
            all,
        } => {
            let resolved_app_id = app_id.as_deref().or(project_app_id).ok_or_else(|| {
                CliError::Config(
                    "app-id required (use --app-id or run `apple-cli init` in your project)".into(),
                )
            })?;

            let path = format!(
                "/apps/{resolved_app_id}/subscriptionGroups?limit={limit}"
            );

            let doc = if *all {
                client.get_all::<SubscriptionGroupAttributes>(&path).await?
            } else {
                client
                    .get_list::<SubscriptionGroupAttributes>(&path)
                    .await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        SubscriptionGroupsCommands::Create { app_id, name } => {
            let resolved_app_id = app_id.as_deref().or(project_app_id).ok_or_else(|| {
                CliError::Config(
                    "app-id required (use --app-id or run `apple-cli init` in your project)".into(),
                )
            })?;

            let body = serde_json::json!({
                "data": {
                    "type": "subscriptionGroups",
                    "attributes": {
                        "referenceName": name,
                    },
                    "relationships": {
                        "app": {
                            "data": {
                                "id": resolved_app_id,
                                "type": "apps"
                            }
                        }
                    }
                }
            });

            let doc = client
                .post::<_, SubscriptionGroupAttributes>("/subscriptionGroups", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        SubscriptionGroupsCommands::Delete { id } => {
            client
                .delete(&format!("/subscriptionGroups/{id}"))
                .await?;
            let output =
                serde_json::json!({ "message": format!("Subscription group {id} deleted") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
