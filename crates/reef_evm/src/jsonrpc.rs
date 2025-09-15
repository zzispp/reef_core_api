use crate::models::{BlockParameter, TransactionObject};
use reef_jsonrpc::types::JsonRpcRequest;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub enum EthereumRpc {
    Call(TransactionObject, BlockParameter),
    EstimateGas(TransactionObject, BlockParameter),
    GasPrice,
    GetBalance(String, BlockParameter),
    GetTransactionReceipt(String),
    GetTransactionByHash(String),
    GetBlockByNumber(String, bool),
    GetBlockNumber,
    ChainId,
    GetTransactionCount(String, BlockParameter),
    SendRawTransaction(String),
    FeeHistory {
        block_count: u64,
        newest_block: BlockParameter,
        reward_percentiles: Vec<u64>,
    },
}

impl EthereumRpc {
    pub fn method_name(&self) -> &'static str {
        match self {
            EthereumRpc::Call(_, _) => "eth_call",
            EthereumRpc::EstimateGas(_, _) => "eth_estimateGas",
            EthereumRpc::GasPrice => "eth_gasPrice",
            EthereumRpc::GetBalance(_, _) => "eth_getBalance",
            EthereumRpc::GetTransactionReceipt(_) => "eth_getTransactionReceipt",
            EthereumRpc::GetTransactionByHash(_) => "eth_getTransactionByHash",
            EthereumRpc::GetBlockByNumber(_, _) => "eth_getBlockByNumber",
            EthereumRpc::GetBlockNumber => "eth_blockNumber",
            EthereumRpc::ChainId => "eth_chainId",
            EthereumRpc::GetTransactionCount(_, _) => "eth_getTransactionCount",
            EthereumRpc::SendRawTransaction(_) => "eth_sendRawTransaction",
            EthereumRpc::FeeHistory { .. } => "eth_feeHistory",
        }
    }
}

impl EthereumRpc {
    pub fn to_request(&self, id: u64) -> JsonRpcRequest {
        let method = self.method_name();
        let params: Vec<Value> = match self {
            EthereumRpc::Call(tx, block) => {
                vec![json!(tx), json!(block.to_string())]
            }
            EthereumRpc::EstimateGas(tx, block) => {
                vec![json!(tx), json!(block.to_string())]
            }
            EthereumRpc::GasPrice => vec![],
            EthereumRpc::GetBalance(address, block) => {
                vec![json!(address), json!(block.to_string())]
            }
            EthereumRpc::GetTransactionReceipt(hash) => {
                vec![json!(hash)]
            }
            EthereumRpc::GetTransactionByHash(hash) => {
                vec![json!(hash)]
            }
            EthereumRpc::GetBlockByNumber(number, full_tx) => {
                vec![json!(number), json!(full_tx)]
            }
            EthereumRpc::GetBlockNumber => vec![],
            EthereumRpc::ChainId => vec![],
            EthereumRpc::GetTransactionCount(address, block) => {
                vec![json!(address), json!(block.to_string())]
            }
            EthereumRpc::SendRawTransaction(data) => {
                vec![json!(data)]
            }
            EthereumRpc::FeeHistory {
                block_count,
                newest_block,
                reward_percentiles,
            } => {
                vec![
                    json!(format!("0x{:x}", block_count)),
                    json!(newest_block.to_string()),
                    json!(reward_percentiles),
                ]
            }
        };

        JsonRpcRequest::new(id, method, params.into())
    }
}

// ERC-20 function selectors
pub const ERC20_NAME_SELECTOR: &str = "0x06fdde03";
pub const ERC20_SYMBOL_SELECTOR: &str = "0x95d89b41";
pub const ERC20_DECIMALS_SELECTOR: &str = "0x313ce567";
pub const ERC20_BALANCE_OF_SELECTOR: &str = "0x70a08231";

// Helper functions for ERC-20 calls
pub fn encode_balance_of_call(address: &str) -> String {
    let address_clean = address.strip_prefix("0x").unwrap_or(address);
    format!(
        "{}000000000000000000000000{:0>40}",
        ERC20_BALANCE_OF_SELECTOR, address_clean
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_ethereum_rpc_method_names() {
        let call_rpc = EthereumRpc::Call(
            TransactionObject::new_call("0x123", "0xdata"),
            BlockParameter::Latest,
        );
        assert_eq!(call_rpc.method_name(), "eth_call");

        let gas_price_rpc = EthereumRpc::GasPrice;
        assert_eq!(gas_price_rpc.method_name(), "eth_gasPrice");

        let balance_rpc = EthereumRpc::GetBalance("0x123".to_string(), BlockParameter::Latest);
        assert_eq!(balance_rpc.method_name(), "eth_getBalance");
    }

    #[test]
    fn test_ethereum_rpc_to_request() {
        let rpc = EthereumRpc::GetBalance("0x123".to_string(), BlockParameter::Latest);
        let request = rpc.to_request(1);

        assert_eq!(request.id, 1);
        assert_eq!(request.method, "eth_getBalance");
        assert_eq!(request.params, json!(["0x123", "latest"]));
    }

    #[test]
    fn test_encode_balance_of_call() {
        let address = "0x123456789abcdef123456789abcdef123456789a";
        let encoded = encode_balance_of_call(address);

        assert!(encoded.starts_with(ERC20_BALANCE_OF_SELECTOR));
        assert!(encoded.contains("123456789abcdef123456789abcdef123456789a"));
        assert_eq!(encoded.len(), 74); // 10 chars (selector) + 64 chars (padded address)
    }

    #[test]
    fn test_erc20_selectors() {
        assert_eq!(ERC20_NAME_SELECTOR, "0x06fdde03");
        assert_eq!(ERC20_SYMBOL_SELECTOR, "0x95d89b41");
        assert_eq!(ERC20_DECIMALS_SELECTOR, "0x313ce567");
        assert_eq!(ERC20_BALANCE_OF_SELECTOR, "0x70a08231");
    }
}
