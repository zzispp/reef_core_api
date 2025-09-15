use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SolanaBalance {
    pub value: u64,
}

impl From<u64> for SolanaBalance {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solana_balance_creation() {
        let balance = SolanaBalance { value: 1000000000 };
        assert_eq!(balance.value, 1000000000);
    }

    #[test]
    fn test_solana_balance_from_u64() {
        let balance: SolanaBalance = 2000000000u64.into();
        assert_eq!(balance.value, 2000000000);
    }

    #[test]
    fn test_solana_balance_serialization() {
        let balance = SolanaBalance { value: 500000000 };
        let json = serde_json::to_string(&balance).unwrap();
        assert!(json.contains("500000000"));

        let deserialized: SolanaBalance = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.value, 500000000);
    }

    #[test]
    fn test_solana_balance_zero() {
        let balance = SolanaBalance { value: 0 };
        assert_eq!(balance.value, 0);
    }

    #[test]
    fn test_solana_balance_clone() {
        let balance1 = SolanaBalance { value: 123456789 };
        let balance2 = balance1.clone();
        assert_eq!(balance1.value, balance2.value);
    }
}
