use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppAttributes {
    pub name: Option<String>,
    pub bundle_id: Option<String>,
    pub sku: Option<String>,
    pub primary_locale: Option<String>,
    pub content_rights_declaration: Option<String>,
    pub is_or_ever_was_made_for_kids: Option<bool>,
    pub available_in_new_territories: Option<bool>,
}
