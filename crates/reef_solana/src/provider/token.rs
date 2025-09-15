use async_trait::async_trait;
use std::error::Error;

use crate::{
    models::Extension,
    provider::token_mapper::{map_token_data_metaplex, map_token_data_spl_token_2022},
    rpc::client::SolanaClient,
};
use primitives::{Asset, ChainToken};

#[async_trait]
impl ChainToken for SolanaClient {
    async fn get_token_data(
        &self,
        token_address: String,
    ) -> Result<Asset, Box<dyn Error + Sync + Send>> {
        let token_info_result = self.get_token_mint_info(&token_address).await?;
        let token_info = token_info_result.info();

        if let Some(extensions) = &token_info.extensions {
            for ext in extensions {
                if let Extension::TokenMetadata(_token_metadata) = ext {
                    return map_token_data_spl_token_2022(
                        self.get_chain(),
                        token_address,
                        &token_info,
                    );
                }
            }
        }

        let metadata = self.get_metaplex_metadata(&token_address).await?;
        map_token_data_metaplex(self.get_chain(), token_address, &token_info, &metadata)
    }

    fn get_is_token_address(&self, token_address: &str) -> bool {
        token_address.len() >= 40
            && token_address.len() <= 60
            && bs58::decode(token_address).into_vec().is_ok()
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
        assert_eq!(asset.chain, primitives::Chain::Solana);
        assert_eq!(asset.contract_address, Some(token_address));
        assert_eq!(asset.decimals, 6);
        assert_eq!(asset.symbol, "USDC");

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
        assert_eq!(asset1.chain, primitives::Chain::Solana);
        assert_eq!(asset1.contract_address, Some(token_addresses[0].clone()));
        assert_eq!(asset1.decimals, 6);
        assert_eq!(asset1.symbol, "USDC");

        dbg!(&assets);
    }
}
