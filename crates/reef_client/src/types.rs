use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, fmt::Debug};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("HTTP error: {status}")]
    HttpError { status: u16 },
}

#[async_trait]
pub trait Client: Send + Sync + Debug {
    async fn get<R>(&self, path: &str) -> Result<R, ClientError>
    where
        R: DeserializeOwned;

    async fn post<T, R>(
        &self,
        path: &str,
        body: &T,
        headers: Option<HashMap<String, String>>,
    ) -> Result<R, ClientError>
    where
        T: Serialize + Send + Sync,
        R: DeserializeOwned;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_error_display() {
        let error1 = ClientError::RequestFailed("Request timeout".to_string());
        assert_eq!(error1.to_string(), "Request failed: Request timeout");

        let error2 = ClientError::HttpError { status: 404 };
        assert_eq!(error2.to_string(), "HTTP error: 404");

        let error3 = ClientError::NetworkError("Connection refused".to_string());
        assert_eq!(error3.to_string(), "Network error: Connection refused");

        let error4 = ClientError::SerializationError("Invalid JSON".to_string());
        assert_eq!(error4.to_string(), "Serialization error: Invalid JSON");
    }

    #[test]
    fn test_client_error_debug() {
        let error = ClientError::RequestFailed("Test error".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("RequestFailed"));
        assert!(debug_str.contains("Test error"));
    }
}
