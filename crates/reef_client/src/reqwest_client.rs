use crate::types::{Client, ClientError};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    base_url: String,
    client: reqwest::Client,
}

impl ReqwestClient {
    pub fn new(base_url: String, client: reqwest::Client) -> Self {
        Self { base_url, client }
    }

    pub fn new_with_url(base_url: String) -> Self {
        let client = reqwest::Client::new();
        Self::new(base_url, client)
    }
}

#[async_trait]
impl Client for ReqwestClient {
    async fn get<R>(&self, path: &str) -> Result<R, ClientError>
    where
        R: DeserializeOwned,
    {
        let url = if path.is_empty() {
            self.base_url.clone()
        } else {
            format!(
                "{}/{}",
                self.base_url.trim_end_matches('/'),
                path.trim_start_matches('/')
            )
        };

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ClientError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ClientError::HttpError {
                status: response.status().as_u16(),
            });
        }

        response
            .json::<R>()
            .await
            .map_err(|e| ClientError::SerializationError(e.to_string()))
    }

    async fn post<T, R>(
        &self,
        path: &str,
        body: &T,
        headers: Option<HashMap<String, String>>,
    ) -> Result<R, ClientError>
    where
        T: Serialize + Send + Sync,
        R: DeserializeOwned,
    {
        let url = if path.is_empty() {
            self.base_url.clone()
        } else {
            format!(
                "{}/{}",
                self.base_url.trim_end_matches('/'),
                path.trim_start_matches('/')
            )
        };

        let mut request = self.client.post(&url).json(body);

        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        let response = request
            .send()
            .await
            .map_err(|e| ClientError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ClientError::HttpError {
                status: response.status().as_u16(),
            });
        }

        response
            .json::<R>()
            .await
            .map_err(|e| ClientError::SerializationError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestRequest {
        message: String,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestResponse {
        echo: String,
    }

    #[test]
    fn test_reqwest_client_creation() {
        let client = ReqwestClient::new_with_url("https://api.example.com".to_string());
        assert_eq!(client.base_url, "https://api.example.com");
    }

    #[test]
    fn test_client_debug() {
        let client = ReqwestClient::new_with_url("https://api.example.com".to_string());
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("ReqwestClient"));
        assert!(debug_str.contains("https://api.example.com"));
    }

    #[test]
    fn test_client_clone() {
        let client = ReqwestClient::new_with_url("https://api.example.com".to_string());
        let cloned = client.clone();
        assert_eq!(client.base_url, cloned.base_url);
    }
}
