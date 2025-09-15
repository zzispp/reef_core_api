use std::error::Error;

use async_trait::async_trait;
use primitives::ChainBalances;
use primitives::{AssetBalance, Balance, Chain};
use tracing::{error, warn};

use crate::models::TokenAccountInfoStruct;
use crate::rpc::client::SolanaClient;

fn map_sol_balance(lamports: u64) -> AssetBalance {
    AssetBalance {
        chain: Chain::Solana,
        contract_address: None,
        balance: Balance::coin_balance(num_bigint::BigUint::from(lamports), 9), // SOL has 9 decimals
        is_active: Some(true),
    }
}

fn map_token_balance(amount: u64, mint: String, decimals: u8) -> AssetBalance {
    AssetBalance::new_token(
        Chain::Solana,
        Some(mint),
        num_bigint::BigUint::from(amount),
        decimals,
    )
}

/// 从代币账户信息解析余额
fn parse_token_account_balance(
    account_info: &TokenAccountInfoStruct,
) -> Result<(String, u64, u8), Box<dyn Error + Send + Sync>> {
    let amount = account_info
        .account
        .data
        .parsed
        .info
        .token_amount
        .amount
        .parse::<u64>()
        .map_err(|e| format!("Failed to parse token amount: {}", e))?;

    let mint = account_info.account.data.parsed.info.mint.clone();
    let decimals = account_info.account.data.parsed.info.token_amount.decimals;

    Ok((mint, amount, decimals))
}

#[async_trait]
impl ChainBalances for SolanaClient {
    async fn get_balance_coin(
        &self,
        address: String,
    ) -> Result<AssetBalance, Box<dyn Error + Sync + Send>> {
        let lamports = self.get_balance(&address).await?;
        Ok(map_sol_balance(lamports))
    }

    async fn get_balance_tokens(
        &self,
        address: String,
        token_addresses: Vec<String>,
    ) -> Result<Vec<AssetBalance>, Box<dyn Error + Sync + Send>> {
        let mut balances = Vec::new();

        for token_address in token_addresses {
            match self
                .get_token_accounts(&address, Some(&token_address))
                .await
            {
                Ok(accounts) => {
                    let mut total_amount = 0u64;
                    let mut decimals = 6u8; // 默认值

                    for account in accounts {
                        match parse_token_account_balance(&account) {
                            Ok((mint, amount, token_decimals)) => {
                                if mint == token_address {
                                    total_amount += amount;
                                    decimals = token_decimals;
                                }
                            }
                            Err(e) => {
                                warn!("解析代币账户数据失败: {:?}", e);
                            }
                        }
                    }

                    if total_amount > 0 {
                        balances.push(map_token_balance(total_amount, token_address, decimals));
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
        let mut balances = Vec::new();

        // 获取SOL余额
        match self.get_balance(&address).await {
            Ok(lamports) => {
                if lamports > 0 {
                    balances.push(map_sol_balance(lamports));
                }
            }
            Err(_) => {
                // 如果获取SOL余额失败，跳过
            }
        }

        // 获取所有代币余额
        match self.get_token_accounts(&address, None).await {
            Ok(accounts) => {
                for account in accounts {
                    match parse_token_account_balance(&account) {
                        Ok((mint, amount, decimals)) => {
                            if amount > 0 {
                                balances.push(map_token_balance(amount, mint, decimals));
                            }
                        }
                        Err(e) => {
                            warn!("解析代币账户数据失败: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                // 如果获取代币账户失败，打印错误但不中断流程
                error!(
                    "❌ Failed to get token accounts for address {}: {:?}",
                    address, e
                );
            }
        }

        Ok(balances)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_sol_balance() {
        let lamports = 1000000000u64; // 1 SOL

        let result = map_sol_balance(lamports);

        assert_eq!(result.chain, Chain::Solana);
        assert_eq!(result.contract_address, None);
        assert_eq!(result.is_active, Some(true));
        assert_eq!(result.balance.amount, lamports.to_string());
        assert_eq!(result.balance.decimals, 9);
        assert_eq!(result.balance.ui_amount, Some(1.0)); // 1000000000 / 10^9 = 1.0
    }

    #[test]
    fn test_map_sol_balance_zero() {
        let lamports = 0u64;

        let result = map_sol_balance(lamports);

        assert_eq!(result.chain, Chain::Solana);
        assert_eq!(result.balance.amount, "0");
        assert_eq!(result.balance.decimals, 9);
        assert_eq!(result.balance.ui_amount, Some(0.0));
    }

    #[test]
    fn test_map_token_balance() {
        let amount = 1000000u64; // 1 token (6 decimals)
        let mint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(); // USDC mint
        let decimals = 6u8;

        let result = map_token_balance(amount, mint.clone(), decimals);

        assert_eq!(result.chain, Chain::Solana);
        assert_eq!(result.contract_address, Some(mint));
        assert_eq!(result.is_active, Some(true));
        assert_eq!(result.balance.amount, amount.to_string());
        assert_eq!(result.balance.decimals, decimals);
        assert_eq!(result.balance.ui_amount, Some(1.0)); // 1000000 / 10^6 = 1.0
    }

    #[test]
    fn test_map_token_balance_zero() {
        let amount = 0u64;
        let mint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string();
        let decimals = 6u8;

        let result = map_token_balance(amount, mint.clone(), decimals);

        assert_eq!(result.chain, Chain::Solana);
        assert_eq!(result.contract_address, Some(mint));
        assert_eq!(result.balance.amount, "0");
        assert_eq!(result.balance.decimals, decimals);
        assert_eq!(result.balance.ui_amount, Some(0.0));
    }
}
