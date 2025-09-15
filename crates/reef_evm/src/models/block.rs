use super::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub number: String,
    pub hash: String,
    #[serde(rename = "parentHash")]
    pub parent_hash: String,
    pub nonce: String,
    #[serde(rename = "sha3Uncles")]
    pub sha3_uncles: String,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: String,
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: String,
    #[serde(rename = "stateRoot")]
    pub state_root: String,
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: String,
    pub miner: String,
    pub difficulty: String,
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: String,
    #[serde(rename = "extraData")]
    pub extra_data: String,
    pub size: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    pub timestamp: String,
    pub transactions: Vec<Transaction>,
    pub uncles: Vec<String>,
}
