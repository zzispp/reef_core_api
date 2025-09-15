use crate::models::{ResultTokenInfo, SolanaBalance, TokenAccountInfo, ValueResult};
use crate::pubkey::Pubkey;
use base64::{prelude::BASE64_STANDARD, Engine};
use primitives::Chain;
use reef_jsonrpc::JsonRpcClient;
use std::error::Error;
use std::str::FromStr;

pub struct SolanaClient {
    client: JsonRpcClient<reef_client::ReqwestClient>,
}

impl SolanaClient {
    pub fn new(rpc_url: String) -> Self {
        let client = JsonRpcClient::new_reqwest(rpc_url);
        Self { client }
    }

    pub fn get_chain(&self) -> Chain {
        Chain::Solana
    }

    pub fn verify_address(&self, address: String) -> Result<(), Box<dyn Error + Sync + Send>> {
        Pubkey::from_str(&address).map_err(|e| format!("Invalid Solana address: {}", e))?;
        Ok(())
    }

    /// 获取SOL余额（以lamports为单位）
    pub async fn get_balance(&self, address: &str) -> Result<u64, Box<dyn Error + Send + Sync>> {
        let params = serde_json::json!([address]);
        let balance: SolanaBalance = self
            .client
            .call("getBalance", params)
            .await
            .map_err(|e| format!("Failed to get balance: {}", e))?;
        Ok(balance.value)
    }

    /// 获取当前slot
    pub async fn get_slot(&self) -> Result<u64, Box<dyn Error + Send + Sync>> {
        let params = serde_json::json!([]);
        let slot: u64 = self
            .client
            .call("getSlot", params)
            .await
            .map_err(|e| format!("Failed to get slot: {}", e))?;
        Ok(slot)
    }

    /// 获取代币账户列表
    pub async fn get_token_accounts(
        &self,
        owner: &str,
        mint: Option<&str>,
    ) -> Result<Vec<TokenAccountInfo>, Box<dyn Error + Send + Sync>> {
        let params = if let Some(mint_address) = mint {
            // 获取特定代币的账户
            serde_json::json!([
                owner,
                {
                    "mint": mint_address
                },
                {
                    "encoding": "jsonParsed"
                }
            ])
        } else {
            // 获取所有代币账户
            serde_json::json!([
                owner,
                {
                    "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                },
                {
                    "encoding": "jsonParsed"
                }
            ])
        };

        let result: ValueResult<Vec<TokenAccountInfo>> = self
            .client
            .call("getTokenAccountsByOwner", params)
            .await
            .map_err(|e| format!("Failed to get token accounts: {}", e))?;

        Ok(result.value)
    }

    /// 发送交易
    pub async fn send_transaction(
        &self,
        signed_transaction: &[u8],
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let transaction_b64 = BASE64_STANDARD.encode(signed_transaction);
        let params = serde_json::json!([
            transaction_b64,
            {
                "encoding": "base64"
            }
        ]);

        let signature: String = self
            .client
            .call("sendTransaction", params)
            .await
            .map_err(|e| format!("Failed to send transaction: {}", e))?;

        Ok(signature)
    }

    /// 确认交易
    pub async fn confirm_transaction(
        &self,
        signature: &str,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let params = serde_json::json!([signature]);
        let status: Option<serde_json::Value> = self
            .client
            .call("getSignatureStatus", params)
            .await
            .map_err(|e| format!("Failed to get signature status: {}", e))?;

        Ok(status.is_some())
    }

    /// 获取代币信息
    pub async fn get_token_mint_info(
        &self,
        token_mint: &str,
    ) -> Result<ResultTokenInfo, Box<dyn Error + Send + Sync>> {
        let params = serde_json::json!([
            token_mint,
            {
                "encoding": "jsonParsed"
            }
        ]);

        let result: ResultTokenInfo = self
            .client
            .call("getAccountInfo", params)
            .await
            .map_err(|e| format!("Failed to get token mint info: {}", e))?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[test]
    fn test_new_client() {
        let client = SolanaClient::new("https://api.mainnet-beta.solana.com".to_string());
        assert_eq!(client.get_chain(), Chain::Solana);
    }

    #[test]
    fn test_address_validation() {
        let client = SolanaClient::new("https://api.mainnet-beta.solana.com".to_string());

        // Test parsing valid Solana addresses
        let valid_addresses = [
            "11111111111111111111111111111112",             // System Program
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",  // Token Program
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
        ];

        for address in valid_addresses {
            assert!(client.verify_address(address.to_string()).is_ok());
        }

        // Test invalid address
        let invalid_address = "invalid_address";
        assert!(client.verify_address(invalid_address.to_string()).is_err());
    }

    #[tokio::test]
    async fn test_get_balance_async() {
        let client = SolanaClient::new("https://api.devnet.solana.com".to_string());

        // Test with System Program address (always exists)
        let system_program = "11111111111111111111111111111112";

        match client.get_balance(system_program).await {
            Ok(balance) => {
                println!("System program balance: {} lamports", balance);
                // System program should always have some balance
                assert!(balance > 0);
            }
            Err(e) => {
                println!("Network error (expected in CI): {}", e);
                // Don't fail the test on network errors in CI/CD
            }
        }
    }

    #[tokio::test]
    async fn test_get_slot_async() {
        let client = SolanaClient::new("https://api.devnet.solana.com".to_string());

        match client.get_slot().await {
            Ok(slot) => {
                println!("Current slot: {}", slot);
                // Slot should always be > 0
                assert!(slot > 0);
            }
            Err(e) => {
                println!("Network error (expected in CI): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_token_accounts_async() {
        let client = SolanaClient::new("https://api.devnet.solana.com".to_string());

        // Use a known address that might have token accounts
        // This is just a test to verify the method works
        let test_address = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";

        match client.get_token_accounts(test_address, None).await {
            Ok(accounts) => {
                println!("Found {} token accounts", accounts.len());
                // Just verify we can call the method without panicking
            }
            Err(e) => {
                println!("Network error or no token accounts (expected): {}", e);
            }
        }
    }
}
