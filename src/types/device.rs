use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAttributes {
    pub name: Option<String>,
    pub platform: Option<String>,
    pub udid: Option<String>,
    pub device_class: Option<String>,
    pub status: Option<String>,
    pub model: Option<String>,
    pub added_date: Option<String>,
}
