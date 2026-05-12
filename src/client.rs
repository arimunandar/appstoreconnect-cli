use crate::auth;
use crate::error::CliError;
use crate::types::common::{Document, ErrorResponse, ListDocument};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;

const BASE_URL: &str = "https://api.appstoreconnect.apple.com/v1";

pub struct ApiClient {
    http: reqwest::Client,
    issuer_id: String,
    key_id: String,
    key_path: String,
}

impl ApiClient {
    pub fn new(issuer_id: &str, key_id: &str, key_path: &str) -> Self {
        Self {
            http: reqwest::Client::new(),
            issuer_id: issuer_id.to_string(),
            key_id: key_id.to_string(),
            key_path: key_path.to_string(),
        }
    }

    fn token(&self) -> Result<String, CliError> {
        auth::generate_token(&self.issuer_id, &self.key_id, &self.key_path)
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<Document<T>, CliError> {
        let url = format!("{BASE_URL}{path}");
        let token = self.token()?;
        let resp = self
            .http
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn get_list<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<ListDocument<T>, CliError> {
        let url = format!("{BASE_URL}{path}");
        self.get_list_url(&url).await
    }

    async fn get_list_url<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<ListDocument<T>, CliError> {
        let token = self.token()?;
        let resp = self
            .http
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn get_all<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<ListDocument<T>, CliError> {
        let url = format!("{BASE_URL}{path}");
        let mut result = self.get_list_url::<T>(&url).await?;
        while let Some(ref links) = result.links {
            if let Some(ref next) = links.next {
                let page = self.get_list_url::<T>(next).await?;
                result.data.extend(page.data);
                result.links = page.links;
                result.meta = page.meta;
            } else {
                break;
            }
        }
        Ok(result)
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<Document<T>, CliError> {
        let url = format!("{BASE_URL}{path}");
        let token = self.token()?;
        let resp = self
            .http
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .header(CONTENT_TYPE, "application/json")
            .json(body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn patch<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<Document<T>, CliError> {
        let url = format!("{BASE_URL}{path}");
        let token = self.token()?;
        let resp = self
            .http
            .patch(&url)
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .header(CONTENT_TYPE, "application/json")
            .json(body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn delete(&self, path: &str) -> Result<(), CliError> {
        let url = format!("{BASE_URL}{path}");
        let token = self.token()?;
        let resp = self
            .http
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            Err(Self::parse_error(status, &body))
        }
    }

    pub async fn post_relationship<B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), CliError> {
        let url = format!("{BASE_URL}{path}");
        let token = self.token()?;
        let resp = self
            .http
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .header(CONTENT_TYPE, "application/json")
            .json(body)
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            Err(Self::parse_error(status, &body))
        }
    }

    #[allow(dead_code)]
    pub async fn delete_relationship<B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), CliError> {
        let url = format!("{BASE_URL}{path}");
        let token = self.token()?;
        let resp = self
            .http
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .header(CONTENT_TYPE, "application/json")
            .json(body)
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            Err(Self::parse_error(status, &body))
        }
    }

    async fn handle_response<T: DeserializeOwned>(
        resp: reqwest::Response,
    ) -> Result<T, CliError> {
        let status = resp.status();
        if status.is_success() {
            let body = resp.text().await?;
            serde_json::from_str(&body).map_err(CliError::from)
        } else {
            let code = status.as_u16();
            let body = resp.text().await.unwrap_or_default();
            Err(Self::parse_error(code, &body))
        }
    }

    fn parse_error(status: u16, body: &str) -> CliError {
        if let Ok(err_resp) = serde_json::from_str::<ErrorResponse>(body) {
            let messages: Vec<String> = err_resp
                .errors
                .iter()
                .map(|e| {
                    if let Some(ref detail) = e.detail {
                        format!("{}: {}", e.title, detail)
                    } else {
                        e.title.clone()
                    }
                })
                .collect();
            CliError::Api {
                status,
                message: messages.join("; "),
            }
        } else {
            CliError::Api {
                status,
                message: body.to_string(),
            }
        }
    }
}
