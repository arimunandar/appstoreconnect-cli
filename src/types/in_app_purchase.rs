use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InAppPurchaseAttributes {
    pub name: Option<String>,
    pub product_id: Option<String>,
    pub in_app_purchase_type: Option<String>,
    pub state: Option<String>,
    pub review_note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InAppPurchaseLocalizationAttributes {
    pub name: Option<String>,
    pub description: Option<String>,
    pub locale: Option<String>,
    pub state: Option<String>,
}
