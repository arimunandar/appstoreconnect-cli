use crate::client::ApiClient;
use crate::error::CliError;
use crate::types::encryption_declaration::AppEncryptionDeclarationAttributes;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum EncryptionDeclarationsCommands {
    /// List encryption declarations for the app
    List {
        #[arg(long)]
        app_id: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        all: bool,
    },
    /// Get an encryption declaration by ID
    Get { id: String },
    /// Create an encryption declaration
    Create {
        #[arg(long)]
        app_id: Option<String>,
        /// Platform: IOS, MAC_OS, TV_OS, VISION_OS
        #[arg(long)]
        platform: String,
        /// Whether the app uses encryption
        #[arg(long)]
        uses_encryption: bool,
        /// Whether the encryption is exempt
        #[arg(long)]
        is_exempt: Option<bool>,
        /// Whether the app contains proprietary cryptography
        #[arg(long)]
        contains_proprietary_cryptography: Option<bool>,
        /// Whether the app contains third-party cryptography
        #[arg(long)]
        contains_third_party_cryptography: Option<bool>,
        /// Whether the app is available on the French store
        #[arg(long)]
        available_on_french_store: Option<bool>,
    },
}

pub async fn execute(
    cmd: &EncryptionDeclarationsCommands,
    client: &ApiClient,
    project_app_id: Option<&str>,
) -> Result<(), CliError> {
    match cmd {
        EncryptionDeclarationsCommands::List {
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
                "/apps/{resolved_app_id}/appEncryptionDeclarations?limit={limit}"
            );

            let doc = if *all {
                client
                    .get_all::<AppEncryptionDeclarationAttributes>(&path)
                    .await?
            } else {
                client
                    .get_list::<AppEncryptionDeclarationAttributes>(&path)
                    .await?
            };
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        EncryptionDeclarationsCommands::Get { id } => {
            let doc = client
                .get::<AppEncryptionDeclarationAttributes>(&format!(
                    "/appEncryptionDeclarations/{id}"
                ))
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
        EncryptionDeclarationsCommands::Create {
            app_id,
            platform,
            uses_encryption,
            is_exempt,
            contains_proprietary_cryptography,
            contains_third_party_cryptography,
            available_on_french_store,
        } => {
            let resolved_app_id = app_id.as_deref().or(project_app_id).ok_or_else(|| {
                CliError::Config(
                    "app-id required (use --app-id or run `apple-cli init` in your project)".into(),
                )
            })?;

            let mut attrs = serde_json::Map::new();
            attrs.insert("platform".into(), serde_json::json!(platform));
            attrs.insert("usesEncryption".into(), serde_json::json!(uses_encryption));
            if let Some(v) = is_exempt {
                attrs.insert("isExempt".into(), serde_json::json!(v));
            }
            if let Some(v) = contains_proprietary_cryptography {
                attrs.insert(
                    "containsProprietaryCryptography".into(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = contains_third_party_cryptography {
                attrs.insert(
                    "containsThirdPartyCryptography".into(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = available_on_french_store {
                attrs.insert("availableOnFrenchStore".into(), serde_json::json!(v));
            }

            let body = serde_json::json!({
                "data": {
                    "type": "appEncryptionDeclarations",
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
                .post::<_, serde_json::Value>("/appEncryptionDeclarations", &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
            Ok(())
        }
    }
}
