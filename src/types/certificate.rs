use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateAttributes {
    pub name: Option<String>,
    pub certificate_type: Option<String>,
    pub display_name: Option<String>,
    pub serial_number: Option<String>,
    pub platform: Option<String>,
    pub expiration_date: Option<String>,
    pub certificate_content: Option<String>,
}
