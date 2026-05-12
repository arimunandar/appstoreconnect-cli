use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::beta_group::BetaGroupAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum BetaGroupsCommands {
    /// List beta groups
    List {
        #[arg(long)]
        filter_app: Option<String>,
        #[arg(long)]
        filter_name: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Get a single beta group by ID
    Get { id: String },
    /// Create a beta group
    Create {
        #[arg(long)]
        app_id: String,
        #[arg(long)]
        name: String,
    },
    /// Update a beta group
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        public_link_enabled: Option<bool>,
        #[arg(long)]
        public_link_limit: Option<i32>,
    },
    /// Delete a beta group
    Delete { id: String },
    /// Add builds to a beta group
    AddBuilds {
        id: String,
        #[arg(long, value_delimiter = ',')]
        build_ids: Vec<String>,
    },
    /// Add testers to a beta group
    AddTesters {
        id: String,
        #[arg(long, value_delimiter = ',')]
        tester_ids: Vec<String>,
    },
}

pub async fn execute(cmd: &BetaGroupsCommands, client: &ApiClient) -> Result<(), CliError> {
    match cmd {
        BetaGroupsCommands::List {
            filter_app,
            filter_name,
            limit,
            all,
        } => {
            let mut params = vec![format!("limit={limit}")];
            if let Some(v) = filter_app {
                params.push(format!("filter[app]={v}"));
            }
            if let Some(v) = filter_name {
                params.push(format!("filter[name]={v}"));
            }
            let path = format!("/betaGroups?{}", params.join("&"));

            let doc = if *all {
                client.get_all::<BetaGroupAttributes>(&path).await?
            } else {
                client.get_list::<BetaGroupAttributes>(&path).await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BetaGroupsCommands::Get { id } => {
            let doc = client
                .get::<BetaGroupAttributes>(&format!("/betaGroups/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BetaGroupsCommands::Create { app_id, name } => {
            let body = serde_json::json!({
                "data": {
                    "type": "betaGroups",
                    "attributes": {
                        "name": name,
                    },
                    "relationships": {
                        "app": {
                            "data": {
                                "id": app_id,
                                "type": "apps"
                            }
                        }
                    }
                }
            });

            let doc = client
                .post::<_, BetaGroupAttributes>("/betaGroups", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BetaGroupsCommands::Update {
            id,
            name,
            public_link_enabled,
            public_link_limit,
        } => {
            let mut attrs = serde_json::Map::new();
            if let Some(v) = name {
                attrs.insert("name".into(), serde_json::json!(v));
            }
            if let Some(v) = public_link_enabled {
                attrs.insert("publicLinkEnabled".into(), serde_json::json!(v));
            }
            if let Some(v) = public_link_limit {
                attrs.insert("publicLinkLimit".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "id": id,
                    "type": "betaGroups",
                    "attributes": attrs,
                }
            });

            let doc = client
                .patch::<_, BetaGroupAttributes>(&format!("/betaGroups/{id}"), &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BetaGroupsCommands::Delete { id } => {
            client.delete(&format!("/betaGroups/{id}")).await?;
            let output = serde_json::json!({ "message": format!("Beta group {id} deleted") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        BetaGroupsCommands::AddBuilds { id, build_ids } => {
            let data: Vec<serde_json::Value> = build_ids
                .iter()
                .map(|bid| serde_json::json!({"id": bid, "type": "builds"}))
                .collect();
            let body = serde_json::json!({ "data": data });
            client
                .post_relationship(&format!("/betaGroups/{id}/relationships/builds"), &body)
                .await?;
            let output = serde_json::json!({ "message": format!("Builds added to beta group {id}") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        BetaGroupsCommands::AddTesters { id, tester_ids } => {
            let data: Vec<serde_json::Value> = tester_ids
                .iter()
                .map(|tid| serde_json::json!({"id": tid, "type": "betaTesters"}))
                .collect();
            let body = serde_json::json!({ "data": data });
            client
                .post_relationship(&format!("/betaGroups/{id}/relationships/betaTesters"), &body)
                .await?;
            let output =
                serde_json::json!({ "message": format!("Testers added to beta group {id}") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
