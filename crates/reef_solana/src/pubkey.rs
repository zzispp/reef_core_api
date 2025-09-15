use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pubkey([u8; 32]);

impl Pubkey {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl Default for Pubkey {
    fn default() -> Self {
        Self([0u8; 32])
    }
}

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(&self.0).into_string())
    }
}

impl FromStr for Pubkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(s)
            .into_vec()
            .map_err(|e| anyhow::anyhow!("Invalid base58: {}", e))?;

        if bytes.len() != 32 {
            return Err(anyhow::anyhow!(
                "Invalid pubkey length: expected 32, got {}",
                bytes.len()
            ));
        }

        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        Ok(Self(array))
    }
}

impl From<[u8; 32]> for Pubkey {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pubkey_creation() {
        let bytes = [1u8; 32];
        let pubkey = Pubkey::new(bytes);
        assert_eq!(pubkey.to_bytes(), bytes);
    }

    #[test]
    fn test_pubkey_default() {
        let pubkey = Pubkey::default();
        assert_eq!(pubkey.to_bytes(), [0u8; 32]);
        assert_eq!(pubkey.to_string(), "11111111111111111111111111111111");
    }

    #[test]
    fn test_pubkey_from_str_valid() {
        // Test with a valid Solana pubkey
        let pubkey_str = "11111111111111111111111111111112";
        let pubkey = Pubkey::from_str(pubkey_str).unwrap();
        assert_eq!(pubkey.to_string(), pubkey_str);
    }

    #[test]
    fn test_pubkey_from_str_invalid() {
        // Test with invalid base58
        assert!(Pubkey::from_str("invalid").is_err());

        // Test with wrong length
        assert!(Pubkey::from_str("1111").is_err());

        // Test with empty string
        assert!(Pubkey::from_str("").is_err());
    }

    #[test]
    fn test_pubkey_display() {
        let bytes = [0u8; 32];
        let pubkey = Pubkey::from(bytes);
        let display_str = pubkey.to_string();
        assert_eq!(display_str, "11111111111111111111111111111111");
    }

    #[test]
    fn test_pubkey_from_bytes() {
        let bytes = [42u8; 32];
        let pubkey = Pubkey::from_bytes(bytes);
        assert_eq!(pubkey.to_bytes(), bytes);
    }

    #[test]
    fn test_pubkey_equality() {
        let bytes1 = [1u8; 32];
        let bytes2 = [1u8; 32];
        let bytes3 = [2u8; 32];

        let pubkey1 = Pubkey::from(bytes1);
        let pubkey2 = Pubkey::from(bytes2);
        let pubkey3 = Pubkey::from(bytes3);

        assert_eq!(pubkey1, pubkey2);
        assert_ne!(pubkey1, pubkey3);
    }

    #[test]
    fn test_pubkey_roundtrip() {
        let original = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
        let pubkey = Pubkey::from_str(original).unwrap();
        let roundtrip = pubkey.to_string();
        assert_eq!(original, roundtrip);
    }

    #[test]
    fn test_pubkey_clone() {
        let pubkey1 = Pubkey::from([1u8; 32]);
        let pubkey2 = pubkey1.clone();
        assert_eq!(pubkey1, pubkey2);
    }
}
