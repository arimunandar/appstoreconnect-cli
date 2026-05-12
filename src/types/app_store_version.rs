use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStoreVersionAttributes {
    pub version_string: Option<String>,
    pub platform: Option<String>,
    pub app_store_state: Option<String>,
    pub app_version_state: Option<String>,
    pub copyright: Option<String>,
    pub review_type: Option<String>,
    pub release_type: Option<String>,
    pub earliest_release_date: Option<String>,
    pub created_date: Option<String>,
}
