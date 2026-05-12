use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleIdAttributes {
    pub name: Option<String>,
    pub identifier: Option<String>,
    pub platform: Option<String>,
    pub seed_id: Option<String>,
}
