use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionGroupAttributes {
    pub reference_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionAttributes {
    pub name: Option<String>,
    pub product_id: Option<String>,
    pub family_sharable: Option<bool>,
    pub state: Option<String>,
    pub subscription_period: Option<String>,
    pub review_note: Option<String>,
    pub group_level: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionLocalizationAttributes {
    pub name: Option<String>,
    pub description: Option<String>,
    pub locale: Option<String>,
    pub state: Option<String>,
}
