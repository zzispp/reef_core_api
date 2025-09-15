use crate::EthereumClient;
use async_trait::async_trait;
use primitives::{
    Asset, AssetBalance, BroadcastOptions, Chain, ChainBalances, ChainProvider, ChainState,
    ChainToken, ChainTraits, ChainTransactions, Transaction, TransactionStateRequest,
    TransactionStatus, TransactionUpdate,
};
use std::error::Error;

/// 从字符串结果解析 ABI 编码的字符串
fn decode_string_result(hex_result: &str) -> Option<String> {
    let hex = hex_result.trim_start_matches("0x");
    if hex.len() < 128 {
        return None;
    }

    // 跳过偏移量 (64 字符) 和长度 (64 字符)，获取实际的字符串数据
    let string_data = &hex[128..];

    // 将十六进制转换为字节
    let bytes = hex::decode(string_data).ok()?;

    // 找到空终止符并转换为字符串
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8(bytes[..end].to_vec()).ok()
}

/// 从字符串结果解析 ABI 编码的无符号整数
fn decode_uint_result(hex_result: &str) -> Option<u32> {
    let hex = hex_result.trim_start_matches("0x");
    if hex.is_empty() {
        return None;
    }

    // 对于 decimals，取最后一个字节
    let last_byte = &hex[hex.len().saturating_sub(2)..];
    u8::from_str_radix(last_byte, 16).ok().map(|v| v as u32)
}

impl ChainProvider for EthereumClient {
    fn get_chain(&self) -> Chain {
        self.chain.to_chain()
    }

    fn verify_address(&self, address: String) -> Result<(), Box<dyn Error + Sync + Send>> {
        if address.starts_with("0x") && address.len() == 42 {
            Ok(())
        } else {
            Err("Invalid Ethereum address format".into())
        }
    }
}

#[async_trait]
impl ChainState for EthereumClient {
}

#[async_trait]
impl ChainTransactions for EthereumClient {
}

impl ChainTraits for EthereumClient {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_string_result() {
        // Test USDC name: "USD Coin"
        let hex = "0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000855534420436f696e000000000000000000000000000000000000000000000000";
        assert_eq!(decode_string_result(hex), Some("USD Coin".to_string()));

        // Test empty/invalid hex
        assert_eq!(decode_string_result("0x"), None);
        assert_eq!(decode_string_result("0x123"), None);
    }

    #[test]
    fn test_decode_uint_result() {
        // Test USDC decimals: 6
        let hex = "0x0000000000000000000000000000000000000000000000000000000000000006";
        assert_eq!(decode_uint_result(hex), Some(6));

        // Test 18 decimals (most ERC20 tokens)
        let hex = "0x0000000000000000000000000000000000000000000000000000000000000012";
        assert_eq!(decode_uint_result(hex), Some(18));

        // Test empty hex
        assert_eq!(decode_uint_result("0x"), None);
    }
}
