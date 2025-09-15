use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub chain: super::Chain,
    pub contract_address: Option<String>,
}
