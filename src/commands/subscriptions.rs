use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::subscription::{SubscriptionAttributes, SubscriptionGroupAttributes};
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SubscriptionsCommands {
    /// List subscriptions for the app (fetches all groups, then subscriptions per group)
    List {
        #[arg(long)]
        app_id: Option<String>,
        #[arg(long)]
        group_id: Option<String>,
    },
    /// Get a subscription by ID
    Get { id: String },
    /// Create a subscription in a group
    Create {
        /// Subscription group ID
        #[arg(long)]
        group_id: String,
        /// Product identifier (e.g. com.app.pro.monthly)
        #[arg(long)]
        product_id: String,
        /// Display name
        #[arg(long)]
        name: String,
        /// Subscription period: ONE_WEEK, ONE_MONTH, TWO_MONTHS, THREE_MONTHS, SIX_MONTHS, ONE_YEAR
        #[arg(long)]
        period: String,
        /// Level within the subscription group (1 = highest)
        #[arg(long)]
        group_level: i32,
        /// Whether the subscription is family sharable
        #[arg(long)]
        family_sharable: Option<bool>,
        /// Review note for App Review
        #[arg(long)]
        review_note: Option<String>,
    },
    /// Delete a subscription
    Delete { id: String },
    /// List available price points for a subscription
    #[command(name = "price-points")]
    PricePoints {
        /// Subscription ID
        id: String,
        /// Filter by territory (e.g. USA, JPN, IDN)
        #[arg(long, default_value = "USA")]
        territory: String,
    },
    /// Set a subscription price
    #[command(name = "set-price")]
    SetPrice {
        /// Subscription ID
        #[arg(long)]
        subscription_id: String,
        /// Customer price (e.g. 4.99)
        #[arg(long)]
        price: String,
        /// Territory (e.g. USA)
        #[arg(long, default_value = "USA")]
        territory: String,
    },
}

pub async fn execute(
    cmd: &SubscriptionsCommands,
    client: &ApiClient,
    project_app_id: Option<&str>,
) -> Result<(), CliError> {
    match cmd {
        SubscriptionsCommands::List { app_id, group_id } => {
            if let Some(gid) = group_id {
                // List subscriptions for a specific group
                let path = format!("/subscriptionGroups/{gid}/subscriptions?limit=50");
                let doc = client.get_all::<SubscriptionAttributes>(&path).await?;
                println!("{}", serde_json::to_string_pretty(&doc)?);
            } else {
                // First get all groups, then list subscriptions per group
                let resolved_app_id =
                    app_id.as_deref().or(project_app_id).ok_or_else(|| {
                        CliError::Config(
                            "app-id required (use --app-id, --group-id, or run `apple-cli init` in your project)"
                                .into(),
                        )
                    })?;

                let groups_path =
                    format!("/apps/{resolved_app_id}/subscriptionGroups?limit=50");
                let groups = client
                    .get_all::<SubscriptionGroupAttributes>(&groups_path)
                    .await?;

                let mut all_subscriptions = Vec::new();
                for group in &groups.data {
                    let subs_path = format!(
                        "/subscriptionGroups/{}/subscriptions?limit=50",
                        group.id
                    );
                    let subs = client
                        .get_all::<SubscriptionAttributes>(&subs_path)
                        .await?;
                    all_subscriptions.extend(subs.data);
                }

                let output = serde_json::json!({
                    "data": all_subscriptions,
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            }
            Ok(())
        }
        SubscriptionsCommands::Get { id } => {
            let doc = client
                .get::<SubscriptionAttributes>(&format!("/subscriptions/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        SubscriptionsCommands::Create {
            group_id,
            product_id,
            name,
            period,
            group_level,
            family_sharable,
            review_note,
        } => {
            let mut attrs = serde_json::Map::new();
            attrs.insert("productId".into(), serde_json::json!(product_id));
            attrs.insert("name".into(), serde_json::json!(name));
            attrs.insert("subscriptionPeriod".into(), serde_json::json!(period));
            attrs.insert("groupLevel".into(), serde_json::json!(group_level));

            if let Some(v) = family_sharable {
                attrs.insert("familySharable".into(), serde_json::json!(v));
            }
            if let Some(v) = review_note {
                attrs.insert("reviewNote".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "type": "subscriptions",
                    "attributes": attrs,
                    "relationships": {
                        "group": {
                            "data": {
                                "id": group_id,
                                "type": "subscriptionGroups"
                            }
                        }
                    }
                }
            });

            let doc = client
                .post::<_, SubscriptionAttributes>("/subscriptions", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        SubscriptionsCommands::Delete { id } => {
            client.delete(&format!("/subscriptions/{id}")).await?;
            let output = serde_json::json!({ "message": format!("Subscription {id} deleted") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        SubscriptionsCommands::PricePoints { id, territory } => {
            let path = format!(
                "/subscriptions/{id}/pricePoints?filter[territory]={territory}&limit=200"
            );
            let mut doc = client.get_all::<serde_json::Value>(&path).await?;
            doc.data.retain(|r| {
                r.attributes
                    .as_ref()
                    .and_then(|a| a.get("customerPrice"))
                    .is_some()
            });
            doc.data.sort_by(|a, b| {
                let pa = a.attributes.as_ref()
                    .and_then(|a| a.get("customerPrice"))
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);
                let pb = b.attributes.as_ref()
                    .and_then(|a| a.get("customerPrice"))
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);
                pa.partial_cmp(&pb).unwrap_or(std::cmp::Ordering::Equal)
            });
            let compact: Vec<_> = doc.data.iter().map(|r| {
                let attrs = r.attributes.as_ref().unwrap();
                serde_json::json!({
                    "id": r.id,
                    "customerPrice": attrs.get("customerPrice"),
                    "proceeds": attrs.get("proceeds"),
                })
            }).collect();
            println!("{}", serde_json::to_string_pretty(&compact)?);
            Ok(())
        }
        SubscriptionsCommands::SetPrice {
            subscription_id,
            price,
            territory,
        } => {
            // Find the price point matching the target price
            let path = format!(
                "/subscriptions/{subscription_id}/pricePoints?filter[territory]={territory}&limit=200"
            );
            let doc = client.get_all::<serde_json::Value>(&path).await?;
            let price_point = doc.data.iter().find(|r| {
                r.attributes
                    .as_ref()
                    .and_then(|a| a.get("customerPrice"))
                    .and_then(|v| v.as_str())
                    == Some(price.as_str())
            });

            let pp_id = match price_point {
                Some(pp) => pp.id.clone(),
                None => {
                    return Err(CliError::Config(format!(
                        "No price point found for ${price} in {territory}. Run `subscriptions price-points {subscription_id}` to see available prices."
                    )));
                }
            };

            let body = serde_json::json!({
                "data": {
                    "type": "subscriptionPrices",
                    "relationships": {
                        "subscription": {
                            "data": {
                                "type": "subscriptions",
                                "id": subscription_id
                            }
                        },
                        "subscriptionPricePoint": {
                            "data": {
                                "type": "subscriptionPricePoints",
                                "id": pp_id
                            }
                        }
                    }
                }
            });

            let result = client
                .post::<_, serde_json::Value>("/subscriptionPrices", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
    }
}
