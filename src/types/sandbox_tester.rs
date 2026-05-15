use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxTesterAttributes {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub acct: Option<String>,
    pub territory: Option<String>,
    pub app_store_territory: Option<String>,
    pub subscription_renewal_rate: Option<String>,
    pub interrupted_purchases: Option<bool>,
}
