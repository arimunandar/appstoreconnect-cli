use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BetaGroupAttributes {
    pub name: Option<String>,
    pub is_internal_group: Option<bool>,
    pub public_link_enabled: Option<bool>,
    pub public_link_id: Option<String>,
    pub public_link_limit_enabled: Option<bool>,
    pub public_link_limit: Option<i32>,
    pub public_link: Option<String>,
    pub created_date: Option<String>,
    pub has_access_to_all_builds: Option<bool>,
}
