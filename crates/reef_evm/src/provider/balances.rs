use std::error::Error;

use async_trait::async_trait;
use primitives::ChainBalances;
use primitives::{AssetBalance, AssetId, Balance, EVMChain};

use crate::rpc::client::EthereumClient;

fn map_balance_coin(
    balance_hex: &str,
    chain: EVMChain,
) -> Result<AssetBalance, Box<dyn Error + Sync + Send>> {
    // Parse hex string to BigUint
    let balance_hex = balance_hex.strip_prefix("0x").unwrap_or(balance_hex);
    let balance_big = if balance_hex.is_empty() {
        num_bigint::BigUint::from(0u32)
    } else {
        num_bigint::BigUint::parse_bytes(balance_hex.as_bytes(), 16)
            .unwrap_or_else(|| num_bigint::BigUint::from(0u32))
    };

    Ok(AssetBalance {
        asset_id: AssetId::native(chain.to_chain()),
        balance: Balance::coin_balance(balance_big, 18), // ETH has 18 decimals
        is_active: Some(true),
    })
}

#[async_trait]
impl ChainBalances for EthereumClient {
    async fn get_balance_coin(
        &self,
        address: String,
    ) -> Result<AssetBalance, Box<dyn Error + Sync + Send>> {
        let balance_hex = self.get_eth_balance(&address).await?;
        map_balance_coin(&balance_hex, self.chain)
    }

    async fn get_balance_tokens(
        &self,
        address: String,
        token_addresses: Vec<String>,
    ) -> Result<Vec<AssetBalance>, Box<dyn Error + Sync + Send>> {
        let mut balances = Vec::new();

        for token_address in token_addresses {
            match self.get_token_balance(&token_address, &address).await {
                Ok(balance_hex) => {
                    // Parse hex balance to BigUint
                    let balance_hex = balance_hex.strip_prefix("0x").unwrap_or(&balance_hex);
                    let balance_big = if balance_hex.is_empty() {
                        num_bigint::BigUint::from(0u32)
                    } else {
                        num_bigint::BigUint::parse_bytes(balance_hex.as_bytes(), 16)
                            .unwrap_or_else(|| num_bigint::BigUint::from(0u32))
                    };

                    // Only add non-zero balances
                    if balance_big > num_bigint::BigUint::from(0u32) {
                        balances.push(AssetBalance {
                            asset_id: AssetId::token(self.chain.to_chain(), &token_address),
                            balance: Balance::coin_balance(balance_big, 18), // Default to 18 decimals for ERC-20
                            is_active: Some(true),
                        });
                    }
                }
                Err(_) => {
                    // Token not found or error, skip
                }
            }
        }

        Ok(balances)
    }

    async fn get_assets_balances(
        &self,
        address: String,
    ) -> Result<Vec<AssetBalance>, Box<dyn Error + Send + Sync>> {
        // Get native token balance
        let balances = vec![self.get_balance_coin(address).await?];

        // For now, just return native balance. In a real implementation,
        // you would fetch all token balances for the address
        Ok(balances)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitives::EVMChain;

    #[test]
    fn test_map_balance_coin() {
        let balance_hex = "0x1bc16d674ec80000"; // 2 ETH in wei
        let chain = EVMChain::Ethereum;

        let result = map_balance_coin(balance_hex, chain).unwrap();

        assert_eq!(result.asset_id.chain, primitives::Chain::Ethereum);
        assert_eq!(result.asset_id.token_address, None);
        assert_eq!(result.is_active, Some(true));
        // 2 ETH = 2000000000000000000 wei
        assert_eq!(result.balance.amount, "2000000000000000000");
        assert_eq!(result.balance.decimals, 18);
        assert_eq!(result.balance.ui_amount, Some(2.0));
    }

    #[test]
    fn test_map_balance_coin_different_value() {
        let balance_hex = "0xde0b6b3a7640000"; // 1 ETH in wei
        let chain = EVMChain::Ethereum;

        let result = map_balance_coin(balance_hex, chain).unwrap();

        assert_eq!(result.asset_id.chain, primitives::Chain::Ethereum);
        assert_eq!(result.balance.amount, "1000000000000000000");
        assert_eq!(result.balance.decimals, 18);
        assert_eq!(result.balance.ui_amount, Some(1.0));
    }

    #[test]
    fn test_map_balance_coin_zero() {
        let balance_hex = "0x0";
        let chain = EVMChain::Polygon;

        let result = map_balance_coin(balance_hex, chain).unwrap();

        assert_eq!(result.asset_id.chain, primitives::Chain::Polygon);
        assert_eq!(result.balance.amount, "0");
        assert_eq!(result.balance.decimals, 18);
        assert_eq!(result.balance.ui_amount, Some(0.0));
    }
}
