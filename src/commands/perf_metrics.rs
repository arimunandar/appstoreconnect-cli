use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::perf_metric::DiagnosticSignatureAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum PerfMetricsCommands {
    /// Get performance and power metrics for an app
    App {
        #[arg(long)]
        app_id: Option<String>,
        /// Filter by metric type: DISK, HANG, BATTERY, LAUNCH, MEMORY, ANIMATION, TERMINATION
        #[arg(long)]
        filter_metric_type: Option<String>,
        /// Filter by platform: IOS, MAC_OS
        #[arg(long)]
        filter_platform: Option<String>,
    },
    /// Get performance and power metrics for a build
    Build {
        /// Build ID
        build_id: String,
        /// Filter by metric type: DISK, HANG, BATTERY, LAUNCH, MEMORY, ANIMATION, TERMINATION
        #[arg(long)]
        filter_metric_type: Option<String>,
        /// Filter by platform: IOS, MAC_OS
        #[arg(long)]
        filter_platform: Option<String>,
    },
    /// List diagnostic signatures for a build
    Diagnostics {
        /// Build ID
        build_id: String,
        /// Filter by diagnostic type: DISK_WRITES, HANGS
        #[arg(long)]
        filter_diagnostic_type: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
    },
    /// Get diagnostic logs for a signature
    Logs {
        /// Diagnostic signature ID
        signature_id: String,
        #[arg(long, default_value = "50")]
        limit: u32,
    },
}

pub async fn execute(
    cmd: &PerfMetricsCommands,
    client: &ApiClient,
    project_app_id: Option<&str>,
) -> Result<(), CliError> {
    match cmd {
        PerfMetricsCommands::App {
            app_id,
            filter_metric_type,
            filter_platform,
        } => {
            let resolved_app_id = app_id.as_deref().or(project_app_id).ok_or_else(|| {
                CliError::Config(
                    "app-id required (use --app-id or run `apple-cli init` in your project)".into(),
                )
            })?;

            let mut path = format!("/apps/{resolved_app_id}/perfPowerMetrics");
            let mut params: Vec<String> = Vec::new();
            if let Some(v) = filter_metric_type {
                params.push(format!("filter[metricType]={v}"));
            }
            if let Some(v) = filter_platform {
                params.push(format!("filter[platform]={v}"));
            }
            if !params.is_empty() {
                path.push('?');
                path.push_str(&params.join("&"));
            }

            let bytes = client.get_raw(&path).await?;
            let json: serde_json::Value = serde_json::from_slice(&bytes)?;
            println!("{}", serde_json::to_string_pretty(&json)?);
            Ok(())
        }
        PerfMetricsCommands::Build {
            build_id,
            filter_metric_type,
            filter_platform,
        } => {
            let mut path = format!("/builds/{build_id}/perfPowerMetrics");
            let mut params: Vec<String> = Vec::new();
            if let Some(v) = filter_metric_type {
                params.push(format!("filter[metricType]={v}"));
            }
            if let Some(v) = filter_platform {
                params.push(format!("filter[platform]={v}"));
            }
            if !params.is_empty() {
                path.push('?');
                path.push_str(&params.join("&"));
            }

            let bytes = client.get_raw(&path).await?;
            let json: serde_json::Value = serde_json::from_slice(&bytes)?;
            println!("{}", serde_json::to_string_pretty(&json)?);
            Ok(())
        }
        PerfMetricsCommands::Diagnostics {
            build_id,
            filter_diagnostic_type,
            limit,
        } => {
            let mut path = format!(
                "/builds/{build_id}/diagnosticSignatures?limit={limit}"
            );
            if let Some(v) = filter_diagnostic_type {
                path.push_str(&format!("&filter[diagnosticType]={v}"));
            }

            let doc = client
                .get_all::<DiagnosticSignatureAttributes>(&path)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        PerfMetricsCommands::Logs {
            signature_id,
            limit,
        } => {
            let path = format!(
                "/diagnosticSignatures/{signature_id}/logs?limit={limit}"
            );
            let doc = client
                .get_all::<serde_json::Value>(&path)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
