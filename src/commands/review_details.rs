use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::review_detail::AppStoreReviewDetailAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ReviewDetailsCommands {
    /// Get an App Store review detail by ID
    Get {
        /// App Store review detail ID
        #[arg(long)]
        id: String,
    },
    /// Create an App Store review detail for a version
    Create {
        /// App Store Version ID (relationship)
        #[arg(long)]
        version_id: String,
        /// Contact first name
        #[arg(long)]
        contact_first_name: Option<String>,
        /// Contact last name
        #[arg(long)]
        contact_last_name: Option<String>,
        /// Contact phone
        #[arg(long)]
        contact_phone: Option<String>,
        /// Contact email
        #[arg(long)]
        contact_email: Option<String>,
        /// Whether a demo account is required
        #[arg(long)]
        demo_account_required: Option<bool>,
        /// Demo account name
        #[arg(long)]
        demo_account_name: Option<String>,
        /// Demo account password
        #[arg(long)]
        demo_account_password: Option<String>,
        /// Notes for the reviewer
        #[arg(long)]
        notes: Option<String>,
    },
    /// Update an App Store review detail
    Update {
        /// App Store review detail ID
        #[arg(long)]
        id: String,
        /// Contact first name
        #[arg(long)]
        contact_first_name: Option<String>,
        /// Contact last name
        #[arg(long)]
        contact_last_name: Option<String>,
        /// Contact phone
        #[arg(long)]
        contact_phone: Option<String>,
        /// Contact email
        #[arg(long)]
        contact_email: Option<String>,
        /// Whether a demo account is required
        #[arg(long)]
        demo_account_required: Option<bool>,
        /// Demo account name
        #[arg(long)]
        demo_account_name: Option<String>,
        /// Demo account password
        #[arg(long)]
        demo_account_password: Option<String>,
        /// Notes for the reviewer
        #[arg(long)]
        notes: Option<String>,
    },
}

pub async fn execute(
    cmd: &ReviewDetailsCommands,
    client: &ApiClient,
) -> Result<(), CliError> {
    match cmd {
        ReviewDetailsCommands::Get { id } => {
            let doc = client
                .get::<AppStoreReviewDetailAttributes>(&format!("/appStoreReviewDetails/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        ReviewDetailsCommands::Create {
            version_id,
            contact_first_name,
            contact_last_name,
            contact_phone,
            contact_email,
            demo_account_required,
            demo_account_name,
            demo_account_password,
            notes,
        } => {
            let mut attrs = serde_json::Map::new();
            if let Some(v) = contact_first_name {
                attrs.insert("contactFirstName".into(), serde_json::json!(v));
            }
            if let Some(v) = contact_last_name {
                attrs.insert("contactLastName".into(), serde_json::json!(v));
            }
            if let Some(v) = contact_phone {
                attrs.insert("contactPhone".into(), serde_json::json!(v));
            }
            if let Some(v) = contact_email {
                attrs.insert("contactEmail".into(), serde_json::json!(v));
            }
            if let Some(v) = demo_account_required {
                attrs.insert("demoAccountRequired".into(), serde_json::json!(v));
            }
            if let Some(v) = demo_account_name {
                attrs.insert("demoAccountName".into(), serde_json::json!(v));
            }
            if let Some(v) = demo_account_password {
                attrs.insert("demoAccountPassword".into(), serde_json::json!(v));
            }
            if let Some(v) = notes {
                attrs.insert("notes".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "type": "appStoreReviewDetails",
                    "attributes": attrs,
                    "relationships": {
                        "appStoreVersion": {
                            "data": {
                                "id": version_id,
                                "type": "appStoreVersions"
                            }
                        }
                    }
                }
            });

            let doc = client
                .post::<_, AppStoreReviewDetailAttributes>("/appStoreReviewDetails", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        ReviewDetailsCommands::Update {
            id,
            contact_first_name,
            contact_last_name,
            contact_phone,
            contact_email,
            demo_account_required,
            demo_account_name,
            demo_account_password,
            notes,
        } => {
            let mut attrs = serde_json::Map::new();
            if let Some(v) = contact_first_name {
                attrs.insert("contactFirstName".into(), serde_json::json!(v));
            }
            if let Some(v) = contact_last_name {
                attrs.insert("contactLastName".into(), serde_json::json!(v));
            }
            if let Some(v) = contact_phone {
                attrs.insert("contactPhone".into(), serde_json::json!(v));
            }
            if let Some(v) = contact_email {
                attrs.insert("contactEmail".into(), serde_json::json!(v));
            }
            if let Some(v) = demo_account_required {
                attrs.insert("demoAccountRequired".into(), serde_json::json!(v));
            }
            if let Some(v) = demo_account_name {
                attrs.insert("demoAccountName".into(), serde_json::json!(v));
            }
            if let Some(v) = demo_account_password {
                attrs.insert("demoAccountPassword".into(), serde_json::json!(v));
            }
            if let Some(v) = notes {
                attrs.insert("notes".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "type": "appStoreReviewDetails",
                    "id": id,
                    "attributes": attrs
                }
            });

            let doc = client
                .patch::<_, AppStoreReviewDetailAttributes>(
                    &format!("/appStoreReviewDetails/{id}"),
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
