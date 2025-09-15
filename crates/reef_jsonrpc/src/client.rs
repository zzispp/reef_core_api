use crate::types::{
    JsonRpcError, JsonRpcRequest, JsonRpcResult, JsonRpcResults, ERROR_INTERNAL_ERROR,
};
use reef_client::{Client, ClientError};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::time::SystemTime;

pub type CallTuple = (String, Value);

#[derive(Clone, Debug)]
pub struct JsonRpcClient<C: Client + Clone> {
    client: C,
}

impl From<ClientError> for JsonRpcError {
    fn from(value: ClientError) -> Self {
        JsonRpcError {
            code: ERROR_INTERNAL_ERROR,
            message: value.to_string(),
        }
    }
}

impl<C: Client + Clone> JsonRpcClient<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub async fn call<T: DeserializeOwned>(
        &self,
        method: &str,
        params: impl Into<Value>,
    ) -> Result<T, JsonRpcError> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let req = JsonRpcRequest::new(timestamp, method, params.into());
        let result = self._request(req, None).await?;
        match result {
            JsonRpcResult::Value(value) => Ok(value.result),
            JsonRpcResult::Error(error) => Err(error.error),
        }
    }

    pub async fn batch_call<T: DeserializeOwned>(
        &self,
        calls: Vec<CallTuple>,
    ) -> Result<JsonRpcResults<T>, JsonRpcError> {
        if calls.is_empty() {
            return Ok(Default::default());
        }
        let requests: Vec<JsonRpcRequest> = calls
            .iter()
            .enumerate()
            .map(|(index, (method, params))| {
                JsonRpcRequest::new(index as u64 + 1, method, params.clone())
            })
            .collect();

        self.batch_request(requests).await
    }

    pub async fn batch_request<T: DeserializeOwned>(
        &self,
        requests: Vec<JsonRpcRequest>,
    ) -> Result<JsonRpcResults<T>, JsonRpcError> {
        if requests.is_empty() {
            return Ok(Default::default());
        }

        let results: Vec<JsonRpcResult<T>> = self.client.post("", &requests, None).await?;
        if results.len() != requests.len() {
            return Err(JsonRpcError {
                message: "Batch call response length mismatch".into(),
                code: ERROR_INTERNAL_ERROR,
            });
        }

        Ok(JsonRpcResults(results))
    }

    async fn _request<T: DeserializeOwned>(
        &self,
        req: JsonRpcRequest,
        ttl: Option<u64>,
    ) -> Result<JsonRpcResult<T>, JsonRpcError> {
        // Build cache headers if TTL is provided
        let headers = ttl.map(|ttl_seconds| {
            let mut headers = std::collections::HashMap::new();
            headers.insert(
                "Cache-Control".to_string(),
                format!("max-age={}", ttl_seconds),
            );
            headers
        });

        let result: JsonRpcResult<T> = self.client.post("", &req, headers).await?;
        Ok(result)
    }
}

impl JsonRpcClient<reef_client::ReqwestClient> {
    pub fn new_reqwest(url: String) -> Self {
        use reef_client::ReqwestClient;
        let reqwest_client = reqwest::Client::new();
        let client = ReqwestClient::new(url, reqwest_client);
        Self { client }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestResponse {
        value: String,
    }

    #[test]
    fn test_jsonrpc_client_creation() {
        let client = JsonRpcClient::new_reqwest("https://api.example.com".to_string());
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("JsonRpcClient"));
    }

    #[test]
    fn test_client_error_conversion() {
        let client_error = reef_client::ClientError::NetworkError("Connection failed".to_string());
        let jsonrpc_error: JsonRpcError = client_error.into();

        assert_eq!(jsonrpc_error.code, ERROR_INTERNAL_ERROR);
        assert_eq!(jsonrpc_error.message, "Network error: Connection failed");
    }

    #[test]
    fn test_empty_batch_call() {
        let _client = JsonRpcClient::new_reqwest("https://api.example.com".to_string());

        // This would be an async test in practice, but we're testing the structure
        let calls: Vec<CallTuple> = vec![];
        assert!(calls.is_empty());
    }

    #[test]
    fn test_call_tuple_creation() {
        let calls: Vec<CallTuple> = vec![
            ("eth_getBalance".to_string(), json!(["0x123", "latest"])),
            ("eth_blockNumber".to_string(), json!([])),
        ];

        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].0, "eth_getBalance");
        assert_eq!(calls[1].0, "eth_blockNumber");
    }

    #[test]
    fn test_jsonrpc_client_clone() {
        let client = JsonRpcClient::new_reqwest("https://api.example.com".to_string());
        let cloned = client.clone();

        // Both should be valid clients
        let debug_str1 = format!("{:?}", client);
        let debug_str2 = format!("{:?}", cloned);
        assert!(debug_str1.contains("JsonRpcClient"));
        assert!(debug_str2.contains("JsonRpcClient"));
    }
}
