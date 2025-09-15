use crate::pubkey::Pubkey;
use async_trait::async_trait;
use primitives::{
    Asset, BroadcastOptions, Chain, Transaction, TransactionStateRequest, TransactionStatus,
    TransactionUpdate,
};
use primitives::{ChainProvider, ChainState, ChainToken, ChainTraits, ChainTransactions};
use std::error::Error;
use std::str::FromStr;

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
    async fn get_chain_id(&self) -> Result<String, Box<dyn Error + Sync + Send>> {
        // Solana doesn't have chain IDs like EVM, so we return a static identifier
        Ok("solana-mainnet".to_string())
    }

    async fn get_block_latest_number(&self) -> Result<u64, Box<dyn Error + Sync + Send>> {
        let slot = self.get_slot().await?;
        Ok(slot)
    }
}

#[async_trait]
impl ChainTransactions for SolanaClient {
    async fn transaction_broadcast(
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
    }
}

#[async_trait]
impl ChainToken for SolanaClient {
    async fn get_token_data(
        &self,
        token_address: String,
    ) -> Result<Asset, Box<dyn Error + Sync + Send>> {
        let result = self.get_token_mint_info(&token_address).await?;

        let token_info = result.value.ok_or("Token mint not found")?;
        let decimals = token_info.data.parsed.info.decimals;

        Ok(Asset {
            name: format!("SPL Token {}", &token_address[0..8]),
            symbol: "SPL".to_string(),
            decimals,
            chain: Chain::Solana,
            contract_address: Some(token_address),
        })
    }

    fn get_is_token_address(&self, token_address: &str) -> bool {
        Pubkey::from_str(token_address).is_ok()
    }

    async fn get_tokens_data(
        &self,
        token_addresses: Vec<String>,
    ) -> Result<Vec<Asset>, Box<dyn Error + Sync + Send>> {
        if token_addresses.is_empty() {
            return Ok(vec![]);
        }

        let mut assets = Vec::new();

        // 批量获取代币信息
        for token_address in token_addresses {
            match self.get_token_data(token_address).await {
                Ok(asset) => assets.push(asset),
                Err(_) => {
                    // 跳过获取失败的代币
                    continue;
                }
            }
        }

        Ok(assets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitives::ChainToken;
    use tokio;

    #[test]
    fn test_get_is_token_address() {
        let client = SolanaClient::new("https://api.mainnet-beta.solana.com".to_string());

        // 测试有效的 Solana 地址
        assert!(client.get_is_token_address("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"));
        assert!(client.get_is_token_address("So11111111111111111111111111111111111111112"));

        // 测试无效的地址
        assert!(!client.get_is_token_address(""));
        assert!(!client.get_is_token_address("invalid"));
        assert!(!client.get_is_token_address("0x1234567890123456789012345678901234567890")); // ETH address

        // 测试边界情况
        assert!(!client.get_is_token_address("a".repeat(31).as_str())); // 太短
        assert!(!client.get_is_token_address("a".repeat(45).as_str())); // 太长
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_token_data_single() {
        let client = SolanaClient::new("https://api.mainnet-beta.solana.com".to_string());
        let token_address = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string();

        let result = client.get_token_data(token_address.clone()).await;
        assert!(result.is_ok());

        let asset = result.unwrap();
        assert_eq!(asset.chain, Chain::Solana);
        assert_eq!(asset.contract_address, Some(token_address));
        assert_eq!(asset.decimals, 6); // 默认值
        assert_eq!(asset.symbol, "SPL");

        dbg!(&asset);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_tokens_data_multiple() {
        let client = SolanaClient::new("https://api.mainnet-beta.solana.com".to_string());
        let token_addresses = vec![
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
            "So11111111111111111111111111111111111111112".to_string(),
        ];

        // 测试获取多个代币数据
        let result = client.get_tokens_data(token_addresses.clone()).await;
        assert!(result.is_ok());

        let assets = result.unwrap();
        assert_eq!(assets.len(), 2);

        // 验证第一个代币
        let asset1 = &assets[0];
        assert_eq!(asset1.chain, Chain::Solana);
        assert_eq!(asset1.contract_address, Some(token_addresses[0].clone()));
        assert_eq!(asset1.decimals, 6); // 默认值
        assert_eq!(asset1.symbol, "SPL");

        // 验证第二个代币
        let asset2 = &assets[1];
        assert_eq!(asset2.chain, Chain::Solana);
        assert_eq!(asset2.contract_address, Some(token_addresses[1].clone()));
        assert_eq!(asset2.decimals, 9); // 默认值
        assert_eq!(asset2.symbol, "SPL");

        dbg!(&assets);
    }
}
