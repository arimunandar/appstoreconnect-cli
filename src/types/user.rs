use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAttributes {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub roles: Option<Vec<String>>,
    pub all_apps_visible: Option<bool>,
    pub provisioning_allowed: Option<bool>,
}
