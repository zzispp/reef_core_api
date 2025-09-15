use std::{collections::HashSet, fmt};

use serde::{Deserialize, Serialize};

use crate::{chain::Chain, AssetSubtype};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId {
    pub chain: Chain,
    pub token_address: Option<String>,
}

impl fmt::Display for AssetId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.token_address {
            Some(address) => write!(f, "{}:{}", self.chain, address),
            None => write!(f, "{}", self.chain),
        }
    }
}

impl AssetId {
    /// 传入链和可选的代币地址
    pub fn new(chain: Chain, token_address: Option<String>) -> Self {
        Self {
            chain,
            token_address,
        }
    }

    /// 创建原生币资产ID
    pub fn native(chain: Chain) -> Self {
        Self {
            chain,
            token_address: None,
        }
    }

    /// 创建代币资产ID
    pub fn token(chain: Chain, token_address: &str) -> Self {
        Self {
            chain,
            token_address: Some(token_address.to_string()),
        }
    }

    /// 是否为原生币
    pub fn is_native(&self) -> bool {
        self.token_address.is_none()
    }

    /// 是否为代币
    pub fn is_token(&self) -> bool {
        self.token_address.is_some()
    }

    /// 获取资产子类型
    pub fn token_subtype(&self) -> AssetSubtype {
        if self.is_native() {
            AssetSubtype::NATIVE
        } else {
            AssetSubtype::TOKEN
        }
    }
}

pub trait AssetIdVecExt {
    fn ids(&self) -> Vec<String>;
    fn ids_set(&self) -> HashSet<AssetId>;
}

impl AssetIdVecExt for Vec<AssetId> {
    fn ids(&self) -> Vec<String> {
        self.iter().map(|x| x.to_string()).collect()
    }

    fn ids_set(&self) -> HashSet<AssetId> {
        self.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_asset() {
        let asset_id = AssetId::native(Chain::Ethereum);
        assert_eq!(asset_id.chain, Chain::Ethereum);
        assert_eq!(asset_id.token_address, None);
        assert!(asset_id.is_native());
        assert!(!asset_id.is_token());
        assert_eq!(asset_id.to_string(), "Ethereum");
    }

    #[test]
    fn test_token_asset() {
        let asset_id = AssetId::token(Chain::Ethereum, "0x1234567890abcdef");
        assert_eq!(asset_id.chain, Chain::Ethereum);
        assert_eq!(
            asset_id.token_address,
            Some("0x1234567890abcdef".to_string())
        );
        assert!(!asset_id.is_native());
        assert!(asset_id.is_token());
        assert_eq!(asset_id.to_string(), "Ethereum:0x1234567890abcdef");
    }

    #[test]
    fn test_display_format() {
        let eth = AssetId::native(Chain::Ethereum);
        assert_eq!(format!("{}", eth), "Ethereum");

        let usdc = AssetId::token(Chain::Ethereum, "0xa0b86a33e6409b3b94e55000000000000000000");
        assert_eq!(
            format!("{}", usdc),
            "Ethereum:0xa0b86a33e6409b3b94e55000000000000000000"
        );
    }
}
