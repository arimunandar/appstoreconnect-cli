use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStoreVersionLocalizationAttributes {
    pub locale: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub marketing_url: Option<String>,
    pub promotional_text: Option<String>,
    pub support_url: Option<String>,
    pub whats_new: Option<String>,
}
