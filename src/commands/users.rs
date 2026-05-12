use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::user::UserAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum UsersCommands {
    /// List users
    List {
        #[arg(long)]
        filter_username: Option<String>,
        #[arg(long)]
        filter_roles: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Get a single user by ID
    Get { id: String },
    /// Update user roles
    Update {
        id: String,
        #[arg(long, value_delimiter = ',')]
        roles: Vec<String>,
    },
    /// Remove a user
    Remove { id: String },
}

pub async fn execute(cmd: &UsersCommands, client: &ApiClient) -> Result<(), CliError> {
    match cmd {
        UsersCommands::List {
            filter_username,
            filter_roles,
            limit,
            all,
        } => {
            let mut params = vec![format!("limit={limit}")];
            if let Some(v) = filter_username {
                params.push(format!("filter[username]={v}"));
            }
            if let Some(v) = filter_roles {
                params.push(format!("filter[roles]={v}"));
            }
            let path = format!("/users?{}", params.join("&"));

            let doc = if *all {
                client.get_all::<UserAttributes>(&path).await?
            } else {
                client.get_list::<UserAttributes>(&path).await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        UsersCommands::Get { id } => {
            let doc = client
                .get::<UserAttributes>(&format!("/users/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        UsersCommands::Update { id, roles } => {
            let body = serde_json::json!({
                "data": {
                    "id": id,
                    "type": "users",
                    "attributes": {
                        "roles": roles,
                    }
                }
            });

            let doc = client
                .patch::<_, UserAttributes>(&format!("/users/{id}"), &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        UsersCommands::Remove { id } => {
            client.delete(&format!("/users/{id}")).await?;
            let output = serde_json::json!({ "message": format!("User {id} removed") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
