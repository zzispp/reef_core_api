use super::{AssetType, Chain};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub chain: Chain,
    pub contract_address: Option<String>,
    pub asset_type: AssetType,
}

impl Asset {
    pub fn new(
        name: String,
        symbol: String,
        decimals: i32,
        chain: Chain,
        contract_address: Option<String>,
        asset_type: AssetType,
    ) -> Self {
        Self {
            name,
            symbol,
            decimals,
            chain,
            contract_address,
            asset_type,
        }
    }
}
