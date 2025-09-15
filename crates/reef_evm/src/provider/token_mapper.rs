use primitives::{Asset, AssetType, Chain};

pub fn map_token_data(
    chain: Chain,
    token_address: String,
    name_hex: String,
    symbol_hex: String,
    decimals_hex: String,
) -> Result<Asset, Box<dyn std::error::Error + Send + Sync>> {
    // Parse name (first 32 bytes after length)
    let name = decode_string_result(&name_hex)
        .unwrap_or_else(|| format!("Token {}", &token_address[0..8.min(token_address.len())]));

    // Parse symbol (first 32 bytes after length)
    let symbol = decode_string_result(&symbol_hex).unwrap_or("TOKEN".to_string());

    // Parse decimals (last byte)
    let decimals = decode_uint_result(&decimals_hex).unwrap_or(18);

    // Determine asset type based on chain
    let asset_type = match chain {
        Chain::Ethereum => AssetType::ERC20,
        Chain::SmartChain => AssetType::BEP20,
        Chain::Polygon => AssetType::ERC20,
        // Chain::Arbitrum => AssetType::ERC20,
        // Chain::Optimism => AssetType::ERC20,
        // Chain::Base => AssetType::ERC20,
        // Chain::AvalancheC => AssetType::ERC20,
        _ => AssetType::TOKEN,
    };

    Ok(Asset::new(
        name,
        symbol,
        decimals,
        chain,
        Some(token_address),
        asset_type,
    ))
}

pub fn map_is_token_address(token_address: &str) -> bool {
    token_address.starts_with("0x") && token_address.len() == 42
}

/// Decode a string result from contract call
fn decode_string_result(hex_result: &str) -> Option<String> {
    let hex = hex_result.trim_start_matches("0x");
    if hex.len() < 128 {
        return None;
    }

    // Skip offset (64 chars) and length (64 chars), get the actual string data
    let string_data = &hex[128..];

    // Convert hex to bytes
    let bytes = hex::decode(string_data).ok()?;

    // Find the null terminator and convert to string
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8(bytes[..end].to_vec()).ok()
}

/// Decode a uint result from contract call
fn decode_uint_result(hex_result: &str) -> Option<i32> {
    let hex = hex_result.trim_start_matches("0x");
    if hex.is_empty() {
        return None;
    }

    // Take the last byte for decimals
    let last_byte = &hex[hex.len().saturating_sub(2)..];
    u8::from_str_radix(last_byte, 16).ok().map(|v| v as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_is_token_address() {
        assert!(map_is_token_address(
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
        ));
        assert!(!map_is_token_address("0x1234"));
        assert!(!map_is_token_address(
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48123"
        ));
        assert!(!map_is_token_address(
            "A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
        ));
        assert!(!map_is_token_address(""));
        assert!(!map_is_token_address("0x"));
    }

    #[test]
    fn test_map_token_data() {
        let token_address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string();
        let chain = Chain::Ethereum;
        let name_hex = "0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000855534420436f696e000000000000000000000000000000000000000000000000".to_string();
        let symbol_hex = "0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000045553444300000000000000000000000000000000000000000000000000000000".to_string();
        let decimals_hex =
            "0x0000000000000000000000000000000000000000000000000000000000000006".to_string();

        let result = map_token_data(
            chain,
            token_address.clone(),
            name_hex,
            symbol_hex,
            decimals_hex,
        )
        .unwrap();

        assert_eq!(result.name, "USD Coin");
        assert_eq!(result.symbol, "USDC");
        assert_eq!(result.decimals, 6);
        assert_eq!(result.chain, Chain::Ethereum);
        assert_eq!(result.contract_address, Some(token_address));
        assert_eq!(result.asset_type, AssetType::ERC20);
    }

    #[test]
    fn test_decode_string_result() {
        let hex = "0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000855534420436f696e000000000000000000000000000000000000000000000000";
        assert_eq!(decode_string_result(hex), Some("USD Coin".to_string()));
    }

    #[test]
    fn test_decode_uint_result() {
        let hex = "0x0000000000000000000000000000000000000000000000000000000000000006";
        assert_eq!(decode_uint_result(hex), Some(6));
    }
}
