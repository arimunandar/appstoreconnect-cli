use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

impl CliError {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "error": {
                "type": self.error_type(),
                "message": self.to_string(),
            }
        })
    }

    fn error_type(&self) -> &'static str {
        match self {
            Self::Config(_) => "config_error",
            Self::Auth(_) => "auth_error",
            Self::Api { .. } => "api_error",
            Self::Http(_) => "http_error",
            Self::Json(_) => "json_error",
            Self::Io(_) => "io_error",
            Self::Other(_) => "error",
        }
    }
}
