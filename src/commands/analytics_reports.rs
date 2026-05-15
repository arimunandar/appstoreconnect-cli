use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::analytics_report::{
    AnalyticsReportAttributes, AnalyticsReportInstanceAttributes,
    AnalyticsReportRequestAttributes, AnalyticsReportSegmentAttributes,
};
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum AnalyticsReportsCommands {
    /// Create an analytics report request for an app
    #[command(name = "request")]
    RequestReports {
        #[arg(long)]
        app_id: Option<String>,
        /// Access type: ONGOING or ONE_TIME_SNAPSHOT
        #[arg(long, default_value = "ONGOING")]
        access_type: String,
    },
    /// List analytics report requests for an app
    #[command(name = "list-requests")]
    ListRequests {
        #[arg(long)]
        app_id: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Get an analytics report request by ID
    #[command(name = "get-request")]
    GetRequest {
        /// Analytics report request ID
        id: String,
    },
    /// Delete an analytics report request
    #[command(name = "delete-request")]
    DeleteRequest {
        /// Analytics report request ID
        id: String,
    },
    /// List reports for an analytics report request
    #[command(name = "list-reports")]
    ListReports {
        /// Analytics report request ID
        #[arg(long)]
        request_id: String,
        /// Filter by category: APP_STORE_ENGAGEMENT, APP_STORE_COMMERCE, APP_USAGE, FRAMEWORK_USAGE, PERFORMANCE
        #[arg(long)]
        filter_category: Option<String>,
    },
    /// List instances for an analytics report
    #[command(name = "list-instances")]
    ListInstances {
        /// Analytics report ID
        #[arg(long)]
        report_id: String,
        /// Filter by granularity: DAILY, WEEKLY, MONTHLY
        #[arg(long)]
        filter_granularity: Option<String>,
        /// Filter by processing date (e.g. 2024-01-15)
        #[arg(long)]
        filter_processing_date: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
    },
    /// Get segments for an analytics report instance
    #[command(name = "get-segments")]
    GetSegments {
        /// Analytics report instance ID
        #[arg(long)]
        instance_id: String,
    },
}

pub async fn execute(
    cmd: &AnalyticsReportsCommands,
    client: &ApiClient,
    project_app_id: Option<&str>,
) -> Result<(), CliError> {
    match cmd {
        AnalyticsReportsCommands::RequestReports {
            app_id,
            access_type,
        } => {
            let resolved_app_id = app_id.as_deref().or(project_app_id).ok_or_else(|| {
                CliError::Config(
                    "app-id required (use --app-id or run `apple-cli init` in your project)".into(),
                )
            })?;

            let body = serde_json::json!({
                "data": {
                    "type": "analyticsReportRequests",
                    "attributes": {
                        "accessType": access_type
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
                .post::<_, AnalyticsReportRequestAttributes>("/analyticsReportRequests", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        AnalyticsReportsCommands::ListRequests {
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
                "/apps/{resolved_app_id}/analyticsReportRequests?limit={limit}"
            );

            let doc = if *all {
                client
                    .get_all::<AnalyticsReportRequestAttributes>(&path)
                    .await?
            } else {
                client
                    .get_list::<AnalyticsReportRequestAttributes>(&path)
                    .await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        AnalyticsReportsCommands::GetRequest { id } => {
            let doc = client
                .get::<AnalyticsReportRequestAttributes>(&format!(
                    "/analyticsReportRequests/{id}"
                ))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        AnalyticsReportsCommands::DeleteRequest { id } => {
            client
                .delete(&format!("/analyticsReportRequests/{id}"))
                .await?;
            let output = serde_json::json!({ "message": format!("Analytics report request {id} deleted") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        AnalyticsReportsCommands::ListReports {
            request_id,
            filter_category,
        } => {
            let mut params: Vec<String> = vec![];
            if let Some(v) = filter_category {
                params.push(format!("filter[category]={v}"));
            }
            let path = if params.is_empty() {
                format!("/analyticsReportRequests/{request_id}/reports")
            } else {
                format!(
                    "/analyticsReportRequests/{request_id}/reports?{}",
                    params.join("&")
                )
            };

            let doc = client
                .get_all::<AnalyticsReportAttributes>(&path)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        AnalyticsReportsCommands::ListInstances {
            report_id,
            filter_granularity,
            filter_processing_date,
            limit,
        } => {
            let mut params = vec![format!("limit={limit}")];
            if let Some(v) = filter_granularity {
                params.push(format!("filter[granularity]={v}"));
            }
            if let Some(v) = filter_processing_date {
                params.push(format!("filter[processingDate]={v}"));
            }
            let path = format!(
                "/analyticsReports/{report_id}/instances?{}",
                params.join("&")
            );

            let doc = client
                .get_all::<AnalyticsReportInstanceAttributes>(&path)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        AnalyticsReportsCommands::GetSegments { instance_id } => {
            let doc = client
                .get_all::<AnalyticsReportSegmentAttributes>(&format!(
                    "/analyticsReportInstances/{instance_id}/segments"
                ))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
