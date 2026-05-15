use crate::client::ApiClient;
use crate::error::CliError;
use clap::Subcommand;
use std::io::Write;

#[derive(Debug, Subcommand)]
pub enum SalesReportsCommands {
    /// Download a sales report
    Sales {
        /// Vendor number
        #[arg(long)]
        vendor_number: String,
        /// Report type: SALES, PRE_ORDER, NEWSSTAND, SUBSCRIPTION, SUBSCRIPTION_EVENT, SUBSCRIBER, SUBSCRIPTION_OFFER_CODE_REDEMPTION
        #[arg(long, default_value = "SALES")]
        report_type: String,
        /// Report sub-type: SUMMARY, DETAILED, OPT_IN
        #[arg(long, default_value = "SUMMARY")]
        report_sub_type: String,
        /// Frequency: DAILY, WEEKLY, MONTHLY, YEARLY
        #[arg(long, default_value = "DAILY")]
        frequency: String,
        /// Report date in YYYY-MM-DD format
        #[arg(long)]
        report_date: Option<String>,
        /// Output file path
        #[arg(long)]
        output: Option<String>,
    },
    /// Download a finance report
    Finance {
        /// Vendor number
        #[arg(long)]
        vendor_number: String,
        /// Region code (e.g. US, EU, JP)
        #[arg(long)]
        region_code: String,
        /// Report type: FINANCIAL
        #[arg(long, default_value = "FINANCIAL")]
        report_type: String,
        /// Report date in YYYY-MM format
        #[arg(long)]
        report_date: String,
        /// Output file path
        #[arg(long)]
        output: Option<String>,
    },
}

pub async fn execute(
    cmd: &SalesReportsCommands,
    client: &ApiClient,
) -> Result<(), CliError> {
    match cmd {
        SalesReportsCommands::Sales {
            vendor_number,
            report_type,
            report_sub_type,
            frequency,
            report_date,
            output,
        } => {
            let mut path = format!(
                "/salesReports?filter[vendorNumber]={vendor_number}\
                 &filter[reportType]={report_type}\
                 &filter[reportSubType]={report_sub_type}\
                 &filter[frequency]={frequency}"
            );
            if let Some(v) = report_date {
                path.push_str(&format!("&filter[reportDate]={v}"));
            }

            let bytes = client.get_raw(&path).await?;
            let file_path = output.as_deref().unwrap_or("sales_report.csv.gz");
            let mut file = std::fs::File::create(file_path)?;
            file.write_all(&bytes)?;

            let output_msg = serde_json::json!({
                "message": format!("Report saved to {file_path}"),
                "size_bytes": bytes.len(),
            });
            println!("{}", serde_json::to_string_pretty(&output_msg)?);
            Ok(())
        }
        SalesReportsCommands::Finance {
            vendor_number,
            region_code,
            report_type,
            report_date,
            output,
        } => {
            let path = format!(
                "/financeReports?filter[vendorNumber]={vendor_number}\
                 &filter[regionCode]={region_code}\
                 &filter[reportType]={report_type}\
                 &filter[reportDate]={report_date}"
            );

            let bytes = client.get_raw(&path).await?;
            let file_path = output.as_deref().unwrap_or("finance_report.csv.gz");
            let mut file = std::fs::File::create(file_path)?;
            file.write_all(&bytes)?;

            let output_msg = serde_json::json!({
                "message": format!("Report saved to {file_path}"),
                "size_bytes": bytes.len(),
            });
            println!("{}", serde_json::to_string_pretty(&output_msg)?);
            Ok(())
        }
    }
}
