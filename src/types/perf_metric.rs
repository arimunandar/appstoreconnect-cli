use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticSignatureAttributes {
    pub diagnostic_type: Option<String>,
    pub signature: Option<String>,
    pub weight: Option<f64>,
    pub insight: Option<serde_json::Value>,
}
