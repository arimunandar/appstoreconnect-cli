use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStoreReviewDetailAttributes {
    pub contact_first_name: Option<String>,
    pub contact_last_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub demo_account_required: Option<bool>,
    pub demo_account_name: Option<String>,
    pub demo_account_password: Option<String>,
    pub notes: Option<String>,
}
