use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleIdCapabilityAttributes {
    pub capability_type: Option<String>,
    pub settings: Option<serde_json::Value>,
}
