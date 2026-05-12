use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::certificate::CertificateAttributes;
use base64::Engine;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum CertificatesCommands {
    /// List certificates
    List {
        #[arg(long)]
        filter_type: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Get a single certificate by ID
    Get { id: String },
    /// Create a certificate from a CSR
    Create {
        /// Certificate type (e.g., IOS_DISTRIBUTION, DEVELOPER_ID_APPLICATION)
        #[arg(long, rename_all = "verbatim")]
        r#type: String,
        /// Path to the CSR file
        #[arg(long)]
        csr_path: String,
    },
    /// Revoke a certificate
    Delete { id: String },
}

pub async fn execute(cmd: &CertificatesCommands, client: &ApiClient) -> Result<(), CliError> {
    match cmd {
        CertificatesCommands::List {
            filter_type,
            limit,
            all,
        } => {
            let mut params = vec![format!("limit={limit}")];
            if let Some(v) = filter_type {
                params.push(format!("filter[certificateType]={v}"));
            }
            let path = format!("/certificates?{}", params.join("&"));

            let doc = if *all {
                client.get_all::<CertificateAttributes>(&path).await?
            } else {
                client.get_list::<CertificateAttributes>(&path).await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        CertificatesCommands::Get { id } => {
            let doc = client
                .get::<CertificateAttributes>(&format!("/certificates/{id}"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        CertificatesCommands::Create { r#type, csr_path } => {
            let csr_bytes = std::fs::read(csr_path).map_err(|e| {
                CliError::Other(format!("cannot read CSR file at '{csr_path}': {e}"))
            })?;
            let csr_b64 = base64::engine::general_purpose::STANDARD.encode(&csr_bytes);

            let body = serde_json::json!({
                "data": {
                    "type": "certificates",
                    "attributes": {
                        "certificateType": r#type,
                        "csrContent": csr_b64,
                    }
                }
            });

            let doc = client
                .post::<_, CertificateAttributes>("/certificates", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        CertificatesCommands::Delete { id } => {
            client.delete(&format!("/certificates/{id}")).await?;
            let output = serde_json::json!({ "message": format!("Certificate {id} revoked") });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
