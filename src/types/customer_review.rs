use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerReviewAttributes {
    pub rating: Option<i32>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub reviewer_nickname: Option<String>,
    pub created_date: Option<String>,
    pub territory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerReviewResponseAttributes {
    pub response_body: Option<String>,
    pub last_modified_date: Option<String>,
    pub state: Option<String>,
}
