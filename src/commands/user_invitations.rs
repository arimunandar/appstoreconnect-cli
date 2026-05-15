use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::user_invitation::UserInvitationAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum UserInvitationsCommands {
    /// List pending user invitations
    List {
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Get a user invitation by ID
    Get {
        /// User invitation ID
        id: String,
    },
    /// Invite a user to App Store Connect
    Invite {
        /// Email address
        #[arg(long)]
        email: String,
        /// First name
        #[arg(long)]
        first_name: String,
        /// Last name
        #[arg(long)]
        last_name: String,
        /// Role: ADMIN, FINANCE, ACCOUNT_HOLDER, SALES, DEVELOPER, APP_MANAGER, etc.
        #[arg(long)]
        role: String,
        /// Whether the user can see all apps (defaults to true)
        #[arg(long)]
        all_apps_visible: Option<bool>,
    },
    /// Cancel a pending user invitation
    Cancel {
        /// User invitation ID
        id: String,
    },
}

pub async fn execute(
    cmd: &UserInvitationsCommands,
    client: &ApiClient,
) -> Result<(), CliError> {
    match cmd {
        UserInvitationsCommands::List { limit, all } => {
            let path = format!("/userInvitations?limit={limit}");

            let doc = if *all {
                client.get_all::<UserInvitationAttributes>(&path).await?
            } else {
                client.get_list::<UserInvitationAttributes>(&path).await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        UserInvitationsCommands::Get { id } => {
            let doc = client
                .get::<UserInvitationAttributes>(&format!("/userInvitations/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        UserInvitationsCommands::Invite {
            email,
            first_name,
            last_name,
            role,
            all_apps_visible,
        } => {
            let visible = all_apps_visible.unwrap_or(true);

            let body = serde_json::json!({
                "data": {
                    "type": "userInvitations",
                    "attributes": {
                        "email": email,
                        "firstName": first_name,
                        "lastName": last_name,
                        "roles": [role],
                        "allAppsVisible": visible
                    }
                }
            });

            let doc = client
                .post::<_, UserInvitationAttributes>("/userInvitations", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        UserInvitationsCommands::Cancel { id } => {
            client.delete(&format!("/userInvitations/{id}")).await?;
            let output =
                serde_json::json!({ "message": format!("User invitation {id} cancelled") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
