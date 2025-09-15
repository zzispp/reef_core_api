use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub hash: String,
    pub block_number: u64,
    pub from: String,
    pub to: String,
    pub value: String,
    pub fee: String,
    pub timestamp: u64,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStateRequest {
    pub hash: String,
    pub chain: super::Chain,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionUpdate {
    pub hash: String,
    pub status: TransactionStatus,
    pub block_number: Option<u64>,
    pub confirmations: u32,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastOptions {
    pub priority: Option<String>,
    pub gas_limit: Option<String>,
}
