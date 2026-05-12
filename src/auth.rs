use crate::error::CliError;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Claims {
    iss: String,
    iat: i64,
    exp: i64,
    aud: String,
}

pub fn generate_token(issuer_id: &str, key_id: &str, key_path: &str) -> Result<String, CliError> {
    let pem = std::fs::read(key_path).map_err(|e| {
        CliError::Auth(format!("cannot read private key at '{key_path}': {e}"))
    })?;

    let key = EncodingKey::from_ec_pem(&pem)
        .map_err(|e| CliError::Auth(format!("invalid EC private key: {e}")))?;

    let now = chrono::Utc::now().timestamp();
    let claims = Claims {
        iss: issuer_id.to_string(),
        iat: now,
        exp: now + 20 * 60,
        aud: "appstoreconnect-v1".to_string(),
    };

    let mut header = Header::new(Algorithm::ES256);
    header.kid = Some(key_id.to_string());
    header.typ = Some("JWT".to_string());

    encode(&header, &claims, &key).map_err(|e| CliError::Auth(format!("JWT signing failed: {e}")))
}
