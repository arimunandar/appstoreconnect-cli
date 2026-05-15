use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::subscription::SubscriptionLocalizationAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SubscriptionLocalizationsCommands {
    /// List localizations for a subscription
    List {
        #[arg(long)]
        subscription_id: String,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Create a localization for a subscription
    Create {
        #[arg(long)]
        subscription_id: String,
        /// Locale code (e.g. en-US, id, ja)
        #[arg(long)]
        locale: String,
        /// Display name
        #[arg(long)]
        name: String,
        /// Description (max 55 characters)
        #[arg(long)]
        description: Option<String>,
    },
}

pub async fn execute(
    cmd: &SubscriptionLocalizationsCommands,
    client: &ApiClient,
) -> Result<(), CliError> {
    match cmd {
        SubscriptionLocalizationsCommands::List {
            subscription_id,
            limit,
            all,
        } => {
            let path = format!(
                "/subscriptions/{subscription_id}/subscriptionLocalizations?limit={limit}"
            );

            let doc = if *all {
                client
                    .get_all::<SubscriptionLocalizationAttributes>(&path)
                    .await?
            } else {
                client
                    .get_list::<SubscriptionLocalizationAttributes>(&path)
                    .await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        SubscriptionLocalizationsCommands::Create {
            subscription_id,
            locale,
            name,
            description,
        } => {
            let mut attrs = serde_json::Map::new();
            attrs.insert("name".into(), serde_json::json!(name));
            attrs.insert("locale".into(), serde_json::json!(locale));

            if let Some(v) = description {
                attrs.insert("description".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "type": "subscriptionLocalizations",
                    "attributes": attrs,
                    "relationships": {
                        "subscription": {
                            "data": {
                                "id": subscription_id,
                                "type": "subscriptions"
                            }
                        }
                    }
                }
            });

            let doc = client
                .post::<_, SubscriptionLocalizationAttributes>(
                    "/subscriptionLocalizations",
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
