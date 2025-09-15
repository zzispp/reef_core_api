use async_trait::async_trait;
use primitives::{
    Asset, BroadcastOptions, Chain, Transaction, TransactionStateRequest, TransactionStatus,
    TransactionUpdate,
};
use primitives::{ChainProvider, ChainState, ChainToken, ChainTraits, ChainTransactions};
use std::error::Error;

use crate::rpc::client::EthereumClient;

// Helper functions to decode ERC-20 contract call results
fn decode_string_result(hex_result: &str) -> Option<String> {
    if hex_result.len() < 2 || !hex_result.starts_with("0x") {
        return None;
    }

    let hex_data = &hex_result[2..];
    if hex_data.len() < 128 {
        // At least 64 bytes for offset + length
        return None;
    }

    // Skip first 64 characters (offset), next 64 characters contain length
    let length_hex = &hex_data[64..128];
    let length = u64::from_str_radix(length_hex, 16).ok()?;

    if length == 0 || hex_data.len() < 128 + (length as usize * 2) {
        return None;
    }

    // Extract the actual string data
    let string_hex = &hex_data[128..128 + (length as usize * 2)];
    let bytes = hex::decode(string_hex).ok()?;
    String::from_utf8(bytes).ok()
}

fn decode_uint_result(hex_result: &str) -> Option<u64> {
    if hex_result.len() < 2 || !hex_result.starts_with("0x") {
        return None;
    }

    let hex_data = &hex_result[2..];
    if hex_data.is_empty() {
        return Some(0);
    }

    // Take last 16 hex characters (8 bytes) for u64
    let relevant_hex = if hex_data.len() >= 16 {
        &hex_data[hex_data.len() - 16..]
    } else {
        hex_data
    };

    u64::from_str_radix(relevant_hex, 16).ok()
}

impl ChainTraits for EthereumClient {}

impl ChainProvider for EthereumClient {
    fn get_chain(&self) -> Chain {
        self.get_chain()
    }

    fn verify_address(&self, address: String) -> Result<(), Box<dyn Error + Sync + Send>> {
        self.verify_address(address)
    }
}

#[async_trait]
impl ChainState for EthereumClient {
    async fn get_chain_id(&self) -> Result<String, Box<dyn Error + Sync + Send>> {
        let chain_id = self.get_chain_id().await?;
        Ok(chain_id)
    }

    async fn get_block_latest_number(&self) -> Result<u64, Box<dyn Error + Sync + Send>> {
        let block_number_hex = self.get_block_number().await?;
        let block_number_hex = block_number_hex
            .strip_prefix("0x")
            .unwrap_or(&block_number_hex);
        let block_number = u64::from_str_radix(block_number_hex, 16)
            .map_err(|e| format!("Failed to parse block number: {}", e))?;
        Ok(block_number)
    }
}

#[async_trait]
impl ChainTransactions for EthereumClient {
    async fn transaction_broadcast(
        &self,
        data: String,
        _options: BroadcastOptions,
    ) -> Result<String, Box<dyn Error + Sync + Send>> {
        let tx_hash = self.send_raw_transaction(&data).await?;
        Ok(format!("{:?}", tx_hash))
    }

    async fn get_transaction_status(
        &self,
        request: TransactionStateRequest,
    ) -> Result<TransactionUpdate, Box<dyn Error + Sync + Send>> {
        match self.get_transaction(&request.hash).await {
            Ok(Some(tx)) => {
                let status = if tx.block_number.is_some() {
                    TransactionStatus::Confirmed
                } else {
                    TransactionStatus::Pending
                };

                Ok(TransactionUpdate {
                    hash: request.hash,
                    status,
                    block_number: tx.block_number,
                    confirmations: if tx.block_number.is_some() { 1 } else { 0 },
                })
            }
            Ok(None) => Ok(TransactionUpdate {
                hash: request.hash,
                status: TransactionStatus::Pending,
                block_number: None,
                confirmations: 0,
            }),
            Err(_) => Ok(TransactionUpdate {
                hash: request.hash,
                status: TransactionStatus::Failed,
                block_number: None,
                confirmations: 0,
            }),
        }
    }

    async fn get_transactions_by_address(
        &self,
        _address: String,
        _limit: Option<usize>,
    ) -> Result<Vec<Transaction>, Box<dyn Error + Sync + Send>> {
        // This would require additional APIs like Etherscan or Alchemy
        // For now, return empty vector
        Ok(vec![])
    }
}

#[async_trait]
impl ChainToken for EthereumClient {
    async fn get_token_data(
        &self,
        token_id: String,
    ) -> Result<Asset, Box<dyn Error + Sync + Send>> {
        // Make parallel calls to get token information
        let calls = vec![
            (
                token_id.clone(),
                crate::jsonrpc::ERC20_NAME_SELECTOR.to_string(),
            ),
            (
                token_id.clone(),
                crate::jsonrpc::ERC20_SYMBOL_SELECTOR.to_string(),
            ),
            (
                token_id.clone(),
                crate::jsonrpc::ERC20_DECIMALS_SELECTOR.to_string(),
            ),
        ];

        match self.batch_contract_calls(calls).await {
            Ok(results) if results.len() == 3 => {
                // Parse name (first 32 bytes after length)
                let name = decode_string_result(&results[0])
                    .unwrap_or_else(|| format!("Token {}", &token_id[0..8.min(token_id.len())]));

                // Parse symbol (first 32 bytes after length)
                let symbol = decode_string_result(&results[1]).unwrap_or("TOKEN".to_string());

                // Parse decimals (last byte)
                let decimals = decode_uint_result(&results[2]).unwrap_or(18) as u8;

                Ok(Asset {
                    name,
                    symbol,
                    decimals,
                    chain: self.get_chain(),
                    contract_address: Some(token_id),
                })
            }
            _ => {
                // Fallback to basic asset if calls fail
                Ok(Asset {
                    name: format!("Token {}", &token_id[0..8.min(token_id.len())]),
                    symbol: "TOKEN".to_string(),
                    decimals: 18,
                    chain: self.get_chain(),
                    contract_address: Some(token_id),
                })
            }
        }
    }

    fn get_is_token_address(&self, token_id: &str) -> bool {
        // Simple check for Ethereum address format
        token_id.starts_with("0x") && token_id.len() == 42
    }
}
