use crate::jsonrpc::{
    encode_balance_of_call, ERC20_DECIMALS_SELECTOR, ERC20_NAME_SELECTOR, ERC20_SYMBOL_SELECTOR,
};
use crate::models::{
     FeeHistory, Transaction, TransactionObject, TransactionReceipt,
};
use primitives::{Chain, EVMChain};
use reef_jsonrpc::JsonRpcClient;
use serde_json::json;
use std::error::Error;

pub struct EthereumClient {
    pub chain: EVMChain,
    pub rpc_url: String,
    pub client: JsonRpcClient<reef_client::ReqwestClient>,
}

impl EthereumClient {
    pub fn new(rpc_url: String, chain: EVMChain) -> Self {
        let client = JsonRpcClient::new_reqwest(rpc_url.clone());
        Self {
            chain,
            rpc_url,
            client,
        }
    }

    pub fn get_chain(&self) -> Chain {
        self.chain.to_chain()
    }

    pub fn verify_address(&self, address: String) -> Result<(), Box<dyn Error + Sync + Send>> {
        // Basic Ethereum address validation
        if !address.starts_with("0x") || address.len() != 42 {
            return Err("Invalid Ethereum address format".into());
        }

        // Check if all characters after 0x are valid hex
        let hex_part = &address[2..];
        if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Invalid hex characters in address".into());
        }

        Ok(())
    }

    pub async fn get_eth_balance(
        &self,
        address: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let params = json!([address, "latest"]);
        let balance: String = self
            .client
            .call("eth_getBalance", params)
            .await
            .map_err(|e| format!("Failed to get balance: {}", e))?;
        Ok(balance)
    }

    pub async fn get_block_number(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let params = json!([]);
        let block_number: String = self
            .client
            .call("eth_blockNumber", params)
            .await
            .map_err(|e| format!("Failed to get block number: {}", e))?;
        Ok(block_number)
    }

    pub async fn get_chain_id(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let params = json!([]);
        let chain_id: String = self
            .client
            .call("eth_chainId", params)
            .await
            .map_err(|e| format!("Failed to get chain ID: {}", e))?;
        Ok(chain_id)
    }

    pub async fn get_transaction(
        &self,
        hash: &str,
    ) -> Result<Option<Transaction>, Box<dyn Error + Send + Sync>> {
        let params = json!([hash]);
        let tx: Option<Transaction> = self
            .client
            .call("eth_getTransactionByHash", params)
            .await
            .map_err(|e| format!("Failed to get transaction: {}", e))?;
        Ok(tx)
    }

    pub async fn get_transaction_receipt(
        &self,
        hash: &str,
    ) -> Result<Option<TransactionReceipt>, Box<dyn Error + Send + Sync>> {
        let params = json!([hash]);
        let receipt: Option<TransactionReceipt> = self
            .client
            .call("eth_getTransactionReceipt", params)
            .await
            .map_err(|e| format!("Failed to get transaction receipt: {}", e))?;
        Ok(receipt)
    }

    pub async fn send_raw_transaction(
        &self,
        data: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let params = json!([data]);
        let tx_hash: String = self
            .client
            .call("eth_sendRawTransaction", params)
            .await
            .map_err(|e| format!("Failed to send transaction: {}", e))?;
        Ok(tx_hash)
    }

    pub async fn call_contract(
        &self,
        to: &str,
        data: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let tx_object = TransactionObject::new_call(to, data);
        let params = json!([tx_object, "latest"]);
        let result: String = self
            .client
            .call("eth_call", params)
            .await
            .map_err(|e| format!("Failed to call contract: {}", e))?;
        Ok(result)
    }

    pub async fn get_transaction_count(
        &self,
        address: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let params = json!([address, "latest"]);
        let count: String = self
            .client
            .call("eth_getTransactionCount", params)
            .await
            .map_err(|e| format!("Failed to get transaction count: {}", e))?;
        Ok(count)
    }

    pub async fn estimate_gas(
        &self,
        tx: &TransactionObject,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let params = json!([tx, "latest"]);
        let gas: String = self
            .client
            .call("eth_estimateGas", params)
            .await
            .map_err(|e| format!("Failed to estimate gas: {}", e))?;
        Ok(gas)
    }

    pub async fn get_gas_price(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let params = json!([]);
        let gas_price: String = self
            .client
            .call("eth_gasPrice", params)
            .await
            .map_err(|e| format!("Failed to get gas price: {}", e))?;
        Ok(gas_price)
    }

    pub async fn get_fee_history(
        &self,
        block_count: u64,
        reward_percentiles: Vec<u64>,
    ) -> Result<FeeHistory, Box<dyn Error + Send + Sync>> {
        let params = json!([format!("0x{:x}", block_count), "latest", reward_percentiles]);
        let fee_history: FeeHistory = self
            .client
            .call("eth_feeHistory", params)
            .await
            .map_err(|e| format!("Failed to get fee history: {}", e))?;
        Ok(fee_history)
    }

    /// Get ERC-20 token balance
    pub async fn get_token_balance(
        &self,
        token_address: &str,
        owner_address: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let data = encode_balance_of_call(owner_address);
        self.call_contract(token_address, &data).await
    }

    /// Get ERC-20 token name
    pub async fn get_token_name(
        &self,
        token_address: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        self.call_contract(token_address, ERC20_NAME_SELECTOR).await
    }

    /// Get ERC-20 token symbol  
    pub async fn get_token_symbol(
        &self,
        token_address: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        self.call_contract(token_address, ERC20_SYMBOL_SELECTOR)
            .await
    }

    /// Get ERC-20 token decimals
    pub async fn get_token_decimals(
        &self,
        token_address: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        self.call_contract(token_address, ERC20_DECIMALS_SELECTOR)
            .await
    }

    /// Batch call multiple contracts
    pub async fn batch_contract_calls(
        &self,
        calls: Vec<(String, String)>,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let rpc_calls: Vec<(String, serde_json::Value)> = calls
            .iter()
            .map(|(to, data)| {
                let tx_object = TransactionObject::new_call(to, data);
                ("eth_call".to_string(), json!([tx_object, "latest"]))
            })
            .collect();

        let results = self.client.batch_call::<String>(rpc_calls).await?;
        Ok(results.extract())
    }
}
