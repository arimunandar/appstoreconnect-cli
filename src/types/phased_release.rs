use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStoreVersionPhasedReleaseAttributes {
    pub phased_release_state: Option<String>,
    pub start_date: Option<String>,
    pub total_pause_duration: Option<i32>,
    pub current_day_number: Option<i32>,
}
