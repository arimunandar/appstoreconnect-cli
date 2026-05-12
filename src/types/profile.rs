use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileAttributes {
    pub name: Option<String>,
    pub platform: Option<String>,
    pub profile_type: Option<String>,
    pub profile_state: Option<String>,
    pub profile_content: Option<String>,
    pub uuid: Option<String>,
    pub created_date: Option<String>,
    pub expiration_date: Option<String>,
}
