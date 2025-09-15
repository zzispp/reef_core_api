use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "gasPrice")]
    pub gas_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub data: String,
}

impl TransactionObject {
    pub fn new_call(to: &str, data: &str) -> Self {
        Self {
            from: None,
            to: to.to_string(),
            gas: None,
            gas_price: None,
            value: None,
            data: data.to_string(),
        }
    }

    pub fn new_call_with_value(to: &str, value: &str, data: &str) -> Self {
        Self {
            from: None,
            to: to.to_string(),
            gas: None,
            gas_price: None,
            value: Some(value.to_string()),
            data: data.to_string(),
        }
    }

    pub fn new_call_with_from(from: &str, to: &str, data: &str) -> Self {
        Self {
            from: Some(from.to_string()),
            to: to.to_string(),
            gas: None,
            gas_price: None,
            value: None,
            data: data.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub nonce: String,
    #[serde(rename = "blockHash")]
    pub block_hash: Option<String>,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<u64>,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<u64>,
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<String>,
    pub gas: String,
    pub input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    pub from: String,
    pub to: Option<String>,
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    pub status: String,
    pub logs: Vec<Log>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "logIndex")]
    pub log_index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockParameter {
    Latest,
    Earliest,
    Pending,
    Safe,
    Finalized,
    Number(String),
}

impl Default for BlockParameter {
    fn default() -> Self {
        BlockParameter::Latest
    }
}

impl ToString for BlockParameter {
    fn to_string(&self) -> String {
        match self {
            BlockParameter::Latest => "latest".to_string(),
            BlockParameter::Earliest => "earliest".to_string(),
            BlockParameter::Pending => "pending".to_string(),
            BlockParameter::Safe => "safe".to_string(),
            BlockParameter::Finalized => "finalized".to_string(),
            BlockParameter::Number(n) => n.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_object_new_call() {
        let tx = TransactionObject::new_call("0x123", "0xabcd");
        assert_eq!(tx.to, "0x123");
        assert_eq!(tx.data, "0xabcd");
        assert_eq!(tx.from, None);
        assert_eq!(tx.value, None);
    }

    #[test]
    fn test_transaction_object_with_value() {
        let tx = TransactionObject::new_call_with_value("0x123", "0x1000", "0xabcd");
        assert_eq!(tx.to, "0x123");
        assert_eq!(tx.data, "0xabcd");
        assert_eq!(tx.value, Some("0x1000".to_string()));
    }

    #[test]
    fn test_transaction_object_with_from() {
        let tx = TransactionObject::new_call_with_from("0x456", "0x123", "0xabcd");
        assert_eq!(tx.from, Some("0x456".to_string()));
        assert_eq!(tx.to, "0x123");
        assert_eq!(tx.data, "0xabcd");
    }

    #[test]
    fn test_block_parameter_to_string() {
        assert_eq!(BlockParameter::Latest.to_string(), "latest");
        assert_eq!(BlockParameter::Earliest.to_string(), "earliest");
        assert_eq!(BlockParameter::Pending.to_string(), "pending");
        assert_eq!(BlockParameter::Safe.to_string(), "safe");
        assert_eq!(BlockParameter::Finalized.to_string(), "finalized");
        assert_eq!(
            BlockParameter::Number("0x123".to_string()).to_string(),
            "0x123"
        );
    }

    #[test]
    fn test_block_parameter_default() {
        let default_param = BlockParameter::default();
        assert_eq!(default_param.to_string(), "latest");
    }

    #[test]
    fn test_transaction_serialization() {
        let tx = Transaction {
            hash: "0x123".to_string(),
            nonce: "0x1".to_string(),
            block_hash: Some("0xabc".to_string()),
            block_number: Some(12345),
            transaction_index: Some(0),
            from: "0x456".to_string(),
            to: Some("0x789".to_string()),
            value: "0x1000".to_string(),
            gas_price: Some("0x20".to_string()),
            gas: "0x5208".to_string(),
            input: "0x".to_string(),
        };

        let json = serde_json::to_string(&tx).unwrap();
        let deserialized: Transaction = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.hash, "0x123");
        assert_eq!(deserialized.block_number, Some(12345));
    }

    #[test]
    fn test_transaction_receipt_serialization() {
        let receipt = TransactionReceipt {
            transaction_hash: "0x123".to_string(),
            transaction_index: "0x0".to_string(),
            block_hash: "0xabc".to_string(),
            block_number: "0x123".to_string(),
            from: "0x456".to_string(),
            to: Some("0x789".to_string()),
            cumulative_gas_used: "0x5208".to_string(),
            gas_used: "0x5208".to_string(),
            status: "0x1".to_string(),
            logs: vec![],
        };

        let json = serde_json::to_string(&receipt).unwrap();
        let deserialized: TransactionReceipt = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.transaction_hash, "0x123");
        assert_eq!(deserialized.status, "0x1");
    }

    #[test]
    fn test_log_creation() {
        let log = Log {
            address: "0x123".to_string(),
            topics: vec!["0xabc".to_string(), "0xdef".to_string()],
            data: "0x123456".to_string(),
            block_number: "0x123".to_string(),
            transaction_hash: "0xabc".to_string(),
            transaction_index: "0x0".to_string(),
            block_hash: "0xdef".to_string(),
            log_index: "0x1".to_string(),
        };

        assert_eq!(log.topics.len(), 2);
        assert_eq!(log.address, "0x123");
    }
}
