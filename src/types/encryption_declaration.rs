use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppEncryptionDeclarationAttributes {
    pub app_description: Option<String>,
    pub created_date: Option<String>,
    pub uses_encryption: Option<bool>,
    pub is_exempt: Option<bool>,
    pub contains_proprietary_cryptography: Option<bool>,
    pub contains_third_party_cryptography: Option<bool>,
    pub available_on_french_store: Option<bool>,
    pub platform: Option<String>,
    pub upload_date: Option<String>,
    pub app_encryption_declaration_state: Option<String>,
    pub code_value: Option<String>,
}
