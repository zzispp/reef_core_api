use primitives::Chain;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub chain: Chain,
    pub url: String,
    pub node_type: NodeType,
    pub alchemy_key: String,
    pub ankr_key: String,
    pub trongrid_key: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NodeType {
    Default,
    Archive,
}

impl ProviderConfig {
    pub fn new(
        chain: Chain,
        url: &str,
        node_type: NodeType,
        alchemy_key: &str,
        ankr_key: &str,
        trongrid_key: &str,
    ) -> Self {
        Self {
            chain,
            url: url.to_string(),
            node_type,
            alchemy_key: alchemy_key.to_string(),
            ankr_key: ankr_key.to_string(),
            trongrid_key: trongrid_key.to_string(),
        }
    }
}
