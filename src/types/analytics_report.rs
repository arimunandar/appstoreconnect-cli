use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsReportRequestAttributes {
    pub access_type: Option<String>,
    pub stale: Option<bool>,
    pub stopped_due_to_inactivity: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsReportAttributes {
    pub category: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsReportInstanceAttributes {
    pub granularity: Option<String>,
    pub processing_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsReportSegmentAttributes {
    pub checksum: Option<String>,
    pub size_in_bytes: Option<i64>,
    pub url: Option<String>,
}
