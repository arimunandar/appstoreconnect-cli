use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildAttributes {
    pub version: Option<String>,
    pub uploaded_date: Option<String>,
    pub expiration_date: Option<String>,
    pub expired: Option<bool>,
    pub min_os_version: Option<String>,
    pub processing_state: Option<String>,
    pub build_audience_type: Option<String>,
    pub uses_non_exempt_encryption: Option<bool>,
}
