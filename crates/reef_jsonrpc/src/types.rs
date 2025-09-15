use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Debug, Display};
use thiserror::Error;

pub const JSONRPC_VERSION: &str = "2.0";

pub const ERROR_INVALID_REQUEST: i32 = -32600;
pub const ERROR_METHOD_NOT_FOUND: i32 = -32601;
pub const ERROR_INVALID_PARAMS: i32 = -32602;
pub const ERROR_INTERNAL_ERROR: i32 = -32603;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: &'static str,
    pub id: u64,
    pub method: String,
    pub params: Value,
}

impl JsonRpcRequest {
    pub fn new(id: u64, method: &str, params: Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION,
            id,
            method: method.into(),
            params,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Error)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

impl Display for JsonRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.message, self.code)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonRpcResponse<T> {
    pub id: u64,
    pub result: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonRpcErrorResponse {
    pub id: u64,
    pub error: JsonRpcError,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JsonRpcResult<T> {
    Value(JsonRpcResponse<T>),
    Error(JsonRpcErrorResponse),
}

impl<T> JsonRpcResult<T> {
    pub fn take(self) -> Result<T, JsonRpcError> {
        match self {
            JsonRpcResult::Value(value) => Ok(value.result),
            JsonRpcResult::Error(error) => Err(error.error),
        }
    }
}

pub struct JsonRpcResults<T>(pub Vec<JsonRpcResult<T>>);

impl<T> JsonRpcResults<T> {
    pub fn extract(self) -> Vec<T> {
        let mut extracted = Vec::new();
        for (i, result) in self.0.into_iter().enumerate() {
            match result {
                JsonRpcResult::Value(response) => {
                    extracted.push(response.result);
                }
                JsonRpcResult::Error(error) => {
                    eprintln!("Batch call error for request {}: {:?}", i, error);
                    // Continue processing other results
                }
            }
        }
        extracted
    }
}

impl<T> Default for JsonRpcResults<T> {
    fn default() -> Self {
        JsonRpcResults(Vec::new())
    }
}

impl<T> From<Vec<JsonRpcResult<T>>> for JsonRpcResults<T> {
    fn from(vec: Vec<JsonRpcResult<T>>) -> Self {
        JsonRpcResults(vec)
    }
}

impl<T> IntoIterator for JsonRpcResults<T> {
    type Item = JsonRpcResult<T>;
    type IntoIter = std::vec::IntoIter<JsonRpcResult<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_jsonrpc_request_creation() {
        let request = JsonRpcRequest::new(1, "test_method", json!(["param1", "param2"]));

        assert_eq!(request.jsonrpc, JSONRPC_VERSION);
        assert_eq!(request.id, 1);
        assert_eq!(request.method, "test_method");
        assert_eq!(request.params, json!(["param1", "param2"]));
    }

    #[test]
    fn test_jsonrpc_error_display() {
        let error = JsonRpcError {
            code: ERROR_INVALID_REQUEST,
            message: "Invalid request".to_string(),
        };

        assert_eq!(error.to_string(), "Invalid request (-32600)");
    }

    #[test]
    fn test_jsonrpc_result_take_success() {
        let response = JsonRpcResponse {
            id: 1,
            result: "success".to_string(),
        };
        let result = JsonRpcResult::Value(response);

        assert_eq!(result.take().unwrap(), "success");
    }

    #[test]
    fn test_jsonrpc_result_take_error() {
        let error_response = JsonRpcErrorResponse {
            id: 1,
            error: JsonRpcError {
                code: ERROR_METHOD_NOT_FOUND,
                message: "Method not found".to_string(),
            },
        };
        let result = JsonRpcResult::<String>::Error(error_response);

        let error = result.take().unwrap_err();
        assert_eq!(error.code, ERROR_METHOD_NOT_FOUND);
        assert_eq!(error.message, "Method not found");
    }

    #[test]
    fn test_jsonrpc_results_extract() {
        let results = vec![
            JsonRpcResult::Value(JsonRpcResponse {
                id: 1,
                result: "value1".to_string(),
            }),
            JsonRpcResult::Error(JsonRpcErrorResponse {
                id: 2,
                error: JsonRpcError {
                    code: -1,
                    message: "error".to_string(),
                },
            }),
            JsonRpcResult::Value(JsonRpcResponse {
                id: 3,
                result: "value2".to_string(),
            }),
        ];

        let json_rpc_results = JsonRpcResults(results);
        let extracted = json_rpc_results.extract();

        assert_eq!(extracted.len(), 2);
        assert_eq!(extracted[0], "value1");
        assert_eq!(extracted[1], "value2");
    }

    #[test]
    fn test_jsonrpc_results_default() {
        let results: JsonRpcResults<String> = Default::default();
        assert_eq!(results.0.len(), 0);
    }

    #[test]
    fn test_jsonrpc_results_from_vec() {
        let vec = vec![JsonRpcResult::Value(JsonRpcResponse {
            id: 1,
            result: "test".to_string(),
        })];
        let results: JsonRpcResults<String> = vec.into();
        assert_eq!(results.0.len(), 1);
    }

    #[test]
    fn test_constants() {
        assert_eq!(JSONRPC_VERSION, "2.0");
        assert_eq!(ERROR_INVALID_REQUEST, -32600);
        assert_eq!(ERROR_METHOD_NOT_FOUND, -32601);
        assert_eq!(ERROR_INVALID_PARAMS, -32602);
        assert_eq!(ERROR_INTERNAL_ERROR, -32603);
    }
}
