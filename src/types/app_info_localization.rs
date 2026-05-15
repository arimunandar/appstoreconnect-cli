use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfoLocalizationAttributes {
    pub locale: Option<String>,
    pub name: Option<String>,
    pub subtitle: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub privacy_policy_text: Option<String>,
    pub privacy_choices_url: Option<String>,
}
