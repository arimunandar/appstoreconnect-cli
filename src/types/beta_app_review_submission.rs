use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BetaAppReviewSubmissionAttributes {
    pub beta_review_state: Option<String>,
    pub submitted_date: Option<String>,
}
