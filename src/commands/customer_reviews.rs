use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::customer_review::{CustomerReviewAttributes, CustomerReviewResponseAttributes};
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum CustomerReviewsCommands {
    /// List customer reviews for an app
    List {
        #[arg(long)]
        app_id: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
        #[arg(long)]
        sort: Option<String>,
        #[arg(long)]
        filter_territory: Option<String>,
        #[arg(long)]
        filter_rating: Option<String>,
    },
    /// Get a single customer review by ID
    Get {
        /// Customer review resource ID
        id: String,
    },
    /// Respond to a customer review
    Respond {
        /// Customer review ID to respond to
        #[arg(long)]
        review_id: String,
        /// Response text
        #[arg(long)]
        body: String,
    },
    /// Delete a customer review response
    #[command(name = "delete-response")]
    DeleteResponse {
        /// Customer review response resource ID
        id: String,
    },
}

pub async fn execute(
    cmd: &CustomerReviewsCommands,
    client: &ApiClient,
    project_app_id: Option<&str>,
) -> Result<(), CliError> {
    match cmd {
        CustomerReviewsCommands::List {
            app_id,
            limit,
            all,
            sort,
            filter_territory,
            filter_rating,
        } => {
            let resolved_app_id =
                app_id.as_deref().or(project_app_id).ok_or_else(|| {
                    CliError::Config(
                        "app-id required (use --app-id or run `apple-cli init` in your project)"
                            .into(),
                    )
                })?;

            let mut params = vec![format!("limit={limit}")];
            let sort_value = sort.as_deref().unwrap_or("-createdDate");
            params.push(format!("sort={sort_value}"));
            if let Some(v) = filter_territory {
                params.push(format!("filter[territory]={v}"));
            }
            if let Some(v) = filter_rating {
                params.push(format!("filter[rating]={v}"));
            }
            let path = format!(
                "/apps/{resolved_app_id}/customerReviews?{}",
                params.join("&")
            );

            let doc = if *all {
                client.get_all::<CustomerReviewAttributes>(&path).await?
            } else {
                client.get_list::<CustomerReviewAttributes>(&path).await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        CustomerReviewsCommands::Get { id } => {
            let doc = client
                .get::<CustomerReviewAttributes>(&format!("/customerReviews/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        CustomerReviewsCommands::Respond { review_id, body } => {
            let payload = serde_json::json!({
                "data": {
                    "type": "customerReviewResponses",
                    "attributes": {
                        "responseBody": body
                    },
                    "relationships": {
                        "review": {
                            "data": {
                                "id": review_id,
                                "type": "customerReviews"
                            }
                        }
                    }
                }
            });

            let doc = client
                .post::<_, CustomerReviewResponseAttributes>("/customerReviewResponses", &payload)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        CustomerReviewsCommands::DeleteResponse { id } => {
            client
                .delete(&format!("/customerReviewResponses/{id}"))
                .await?;
            let output =
                serde_json::json!({ "message": format!("Customer review response {id} deleted") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
