use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSubmissionAttributes {
    pub platform: Option<String>,
    pub state: Option<String>,
    pub submitted_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSubmissionItemAttributes {
    pub state: Option<String>,
}
