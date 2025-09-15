use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeHistory {
    #[serde(rename = "oldestBlock")]
    pub oldest_block: String,
    #[serde(rename = "baseFeePerGas")]
    pub base_fee_per_gas: Vec<String>,
    #[serde(rename = "gasUsedRatio")]
    pub gas_used_ratio: Vec<f64>,
    pub reward: Option<Vec<Vec<String>>>,
}
