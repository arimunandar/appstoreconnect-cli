use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::beta_app_review_submission::BetaAppReviewSubmissionAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum BetaAppReviewSubmissionsCommands {
    /// List beta app review submissions
    List {
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
        /// Filter by build ID
        #[arg(long)]
        filter_build: Option<String>,
    },
    /// Get a beta app review submission by ID
    Get {
        /// Submission ID
        id: String,
    },
    /// Submit a build for beta app review
    Submit {
        /// Build ID
        #[arg(long)]
        build_id: String,
    },
}

pub async fn execute(
    cmd: &BetaAppReviewSubmissionsCommands,
    client: &ApiClient,
) -> Result<(), CliError> {
    match cmd {
        BetaAppReviewSubmissionsCommands::List {
            limit,
            all,
            filter_build,
        } => {
            let mut params = vec![format!("limit={limit}")];
            if let Some(v) = filter_build {
                params.push(format!("filter[build]={v}"));
            }
            let path = format!("/betaAppReviewSubmissions?{}", params.join("&"));

            let doc = if *all {
                client
                    .get_all::<BetaAppReviewSubmissionAttributes>(&path)
                    .await?
            } else {
                client
                    .get_list::<BetaAppReviewSubmissionAttributes>(&path)
                    .await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BetaAppReviewSubmissionsCommands::Get { id } => {
            let doc = client
                .get::<BetaAppReviewSubmissionAttributes>(&format!(
                    "/betaAppReviewSubmissions/{id}"
                ))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        BetaAppReviewSubmissionsCommands::Submit { build_id } => {
            let body = serde_json::json!({
                "data": {
                    "type": "betaAppReviewSubmissions",
                    "relationships": {
                        "build": {
                            "data": {
                                "id": build_id,
                                "type": "builds"
                            }
                        }
                    }
                }
            });

            let doc = client
                .post::<_, BetaAppReviewSubmissionAttributes>(
                    "/betaAppReviewSubmissions",
                    &body,
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
