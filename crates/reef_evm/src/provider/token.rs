use std::error::Error;

use crate::provider::token_mapper::{map_is_token_address, map_token_data};
use crate::rpc::client::EthereumClient;

use async_trait::async_trait;
use primitives::{Asset, ChainToken};

#[async_trait]
impl ChainToken for EthereumClient {
    async fn get_token_data(
        &self,
        token_address: String,
    ) -> Result<Asset, Box<dyn Error + Sync + Send>> {
        let calls = vec![
            (
                token_address.clone(),
                crate::jsonrpc::ERC20_NAME_SELECTOR.to_string(),
            ),
            (
                token_address.clone(),
                crate::jsonrpc::ERC20_SYMBOL_SELECTOR.to_string(),
            ),
            (
                token_address.clone(),
                crate::jsonrpc::ERC20_DECIMALS_SELECTOR.to_string(),
            ),
        ];

        let results = self.batch_contract_calls(calls).await?;

        if results.len() != 3 {
            return Err("Failed to get complete token data".into());
        }

        map_token_data(
            self.get_chain(),
            token_address,
            results[0].clone(),
            results[1].clone(),
            results[2].clone(),
        )
    }

    fn get_is_token_address(&self, token_address: &str) -> bool {
        map_is_token_address(token_address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitives::{AssetType, Chain, EVMChain};

    #[test]
    fn test_get_is_token_address() {
        let client = EthereumClient::new("http://localhost:8545".to_string(), EVMChain::Ethereum);

        // Valid Ethereum addresses
        assert!(client.get_is_token_address("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"));
        assert!(client.get_is_token_address("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"));

        // Invalid addresses
        assert!(!client.get_is_token_address("0x1234"));
        assert!(!client.get_is_token_address("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48123"));
        assert!(!client.get_is_token_address("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"));
        assert!(!client.get_is_token_address(""));
        assert!(!client.get_is_token_address("0x"));
    }
}
