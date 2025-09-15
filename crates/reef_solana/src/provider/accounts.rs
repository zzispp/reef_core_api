use async_trait::async_trait;
use primitives::{
     BroadcastOptions, Chain, Transaction, TransactionStateRequest, TransactionStatus,
    TransactionUpdate,
};
use primitives::{ChainProvider, ChainState, ChainToken, ChainTraits, ChainTransactions};
use std::error::Error;

use crate::rpc::client::SolanaClient;
use base64::prelude::*;

impl ChainTraits for SolanaClient {}

impl ChainProvider for SolanaClient {
    fn get_chain(&self) -> Chain {
        self.get_chain()
    }

    fn verify_address(&self, address: String) -> Result<(), Box<dyn Error + Sync + Send>> {
        self.verify_address(address)
    }
}

#[async_trait]
impl ChainState for SolanaClient {
    // async fn get_chain_id(&self) -> Result<String, Box<dyn Error + Sync + Send>> {
    //     // Solana doesn't have chain IDs like EVM, so we return a static identifier
    //     Ok("solana-mainnet".to_string())
    // }
    // 
    // async fn get_block_latest_number(&self) -> Result<u64, Box<dyn Error + Sync + Send>> {
    //     let slot = self.get_slot().await?;
    //     Ok(slot)
    // }
}

#[async_trait]
impl ChainTransactions for SolanaClient {
    /*async fn transaction_broadcast(
        &self,
        data: String,
        _options: BroadcastOptions,
    ) -> Result<String, Box<dyn Error + Sync + Send>> {
        // 将base64字符串转换为字节数组
        let transaction_bytes = base64::prelude::BASE64_STANDARD
            .decode(data)
            .map_err(|e| format!("Failed to decode base64 transaction: {}", e))?;
        let signature = self.send_transaction(&transaction_bytes).await?;
        Ok(signature)
    }

    async fn get_transaction_status(
        &self,
        request: TransactionStateRequest,
    ) -> Result<TransactionUpdate, Box<dyn Error + Sync + Send>> {
        match self.confirm_transaction(&request.hash).await {
            Ok(confirmed) => {
                let status = if confirmed {
                    TransactionStatus::Confirmed
                } else {
                    TransactionStatus::Pending
                };

                Ok(TransactionUpdate {
                    hash: request.hash,
                    status,
                    block_number: None, // Solana uses slots instead of block numbers
                    confirmations: if confirmed { 1 } else { 0 },
                })
            }
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
        // This would require getting transaction signatures and then transaction details
        // For now, return empty vector
        Ok(vec![])
    }*/
}
