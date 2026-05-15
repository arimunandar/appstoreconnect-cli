use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::in_app_purchase::{InAppPurchaseAttributes, InAppPurchaseLocalizationAttributes};
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum InAppPurchasesCommands {
    /// List in-app purchases for the app
    List {
        #[arg(long)]
        app_id: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Get an in-app purchase by ID
    Get { id: String },
    /// Create an in-app purchase
    Create {
        #[arg(long)]
        app_id: Option<String>,
        /// Display name
        #[arg(long)]
        name: String,
        /// Product identifier (e.g. com.app.coins100)
        #[arg(long)]
        product_id: String,
        /// Type: CONSUMABLE, NON_CONSUMABLE, NON_RENEWING_SUBSCRIPTION
        #[arg(long)]
        in_app_purchase_type: String,
    },
    /// Update an in-app purchase
    Update {
        /// In-app purchase ID
        id: String,
        /// Display name
        #[arg(long)]
        name: Option<String>,
        /// Review note for App Review
        #[arg(long)]
        review_note: Option<String>,
    },
    /// Delete an in-app purchase
    Delete { id: String },
    /// List localizations for an in-app purchase
    Localizations {
        /// In-app purchase ID
        id: String,
    },
    /// Create a localization for an in-app purchase
    #[command(name = "create-localization")]
    CreateLocalization {
        /// In-app purchase ID
        #[arg(long)]
        iap_id: String,
        /// Locale code (e.g. en-US, id, ja)
        #[arg(long)]
        locale: String,
        /// Display name
        #[arg(long)]
        name: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
    },
}

pub async fn execute(
    cmd: &InAppPurchasesCommands,
    client: &ApiClient,
    project_app_id: Option<&str>,
) -> Result<(), CliError> {
    match cmd {
        InAppPurchasesCommands::List {
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
                "/apps/{resolved_app_id}/inAppPurchasesV2?limit={limit}"
            );

            let doc = if *all {
                client.get_all::<InAppPurchaseAttributes>(&path).await?
            } else {
                client
                    .get_list::<InAppPurchaseAttributes>(&path)
                    .await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        InAppPurchasesCommands::Get { id } => {
            let doc = client
                .get::<InAppPurchaseAttributes>(&format!("/inAppPurchases/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        InAppPurchasesCommands::Create {
            app_id,
            name,
            product_id,
            in_app_purchase_type,
        } => {
            let resolved_app_id = app_id.as_deref().or(project_app_id).ok_or_else(|| {
                CliError::Config(
                    "app-id required (use --app-id or run `apple-cli init` in your project)".into(),
                )
            })?;

            let mut attrs = serde_json::Map::new();
            attrs.insert("name".into(), serde_json::json!(name));
            attrs.insert("productId".into(), serde_json::json!(product_id));
            attrs.insert(
                "inAppPurchaseType".into(),
                serde_json::json!(in_app_purchase_type),
            );

            let body = serde_json::json!({
                "data": {
                    "type": "inAppPurchases",
                    "attributes": attrs,
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
                .post::<_, serde_json::Value>("/inAppPurchases", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        InAppPurchasesCommands::Update {
            id,
            name,
            review_note,
        } => {
            let mut attrs = serde_json::Map::new();
            if let Some(v) = name {
                attrs.insert("name".into(), serde_json::json!(v));
            }
            if let Some(v) = review_note {
                attrs.insert("reviewNote".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "type": "inAppPurchases",
                    "id": id,
                    "attributes": attrs,
                }
            });

            let doc = client
                .patch::<_, serde_json::Value>(&format!("/inAppPurchases/{id}"), &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        InAppPurchasesCommands::Delete { id } => {
            client
                .delete(&format!("/inAppPurchases/{id}"))
                .await?;
            let output =
                serde_json::json!({ "message": format!("In-app purchase {id} deleted") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        InAppPurchasesCommands::Localizations { id } => {
            let path = format!(
                "/inAppPurchases/{id}/inAppPurchaseLocalizations"
            );
            let doc = client
                .get_all::<InAppPurchaseLocalizationAttributes>(&path)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        InAppPurchasesCommands::CreateLocalization {
            iap_id,
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
                    "type": "inAppPurchaseLocalizations",
                    "attributes": attrs,
                    "relationships": {
                        "inAppPurchase": {
                            "data": {
                                "id": iap_id,
                                "type": "inAppPurchases"
                            }
                        }
                    }
                }
            });

            let doc = client
                .post::<_, InAppPurchaseLocalizationAttributes>(
                    "/inAppPurchaseLocalizations",
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
