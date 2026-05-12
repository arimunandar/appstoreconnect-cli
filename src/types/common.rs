use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document<T> {
    pub data: Resource<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListDocument<T> {
    pub data: Vec<Resource<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<DocumentLinks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource<T> {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentLinks {
    #[serde(rename = "self")]
    pub self_link: Option<String>,
    pub next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<ApiError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub status: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
