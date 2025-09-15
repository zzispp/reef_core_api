use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenAccountInfo {
    pub pubkey: String,
    pub account: TokenAccount,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenAccount {
    pub data: TokenAccountData,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenAccountData {
    pub parsed: ParsedTokenAccount,
    pub program: String,
    pub space: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParsedTokenAccount {
    pub info: TokenAccountParsedInfo,
    #[serde(rename = "type")]
    pub account_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenAccountParsedInfo {
    #[serde(rename = "isNative")]
    pub is_native: bool,
    pub mint: String,
    pub owner: String,
    pub state: String,
    #[serde(rename = "tokenAmount")]
    pub token_amount: TokenAmount,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenAmount {
    pub amount: String,
    pub decimals: u8,
    #[serde(rename = "uiAmount")]
    pub ui_amount: Option<f64>,
    #[serde(rename = "uiAmountString")]
    pub ui_amount_string: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_amount_creation() {
        let token_amount = TokenAmount {
            amount: "1000000".to_string(),
            decimals: 6,
            ui_amount: Some(1.0),
            ui_amount_string: Some("1.0".to_string()),
        };

        assert_eq!(token_amount.amount, "1000000");
        assert_eq!(token_amount.decimals, 6);
        assert_eq!(token_amount.ui_amount, Some(1.0));
    }

    #[test]
    fn test_token_account_serialization() {
        let token_account_info = TokenAccountInfo {
            pubkey: "TokenAccountAddress123".to_string(),
            account: TokenAccount {
                data: TokenAccountData {
                    parsed: ParsedTokenAccount {
                        info: TokenAccountParsedInfo {
                            is_native: false,
                            mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
                            owner: "OwnerAddress123".to_string(),
                            state: "initialized".to_string(),
                            token_amount: TokenAmount {
                                amount: "1000000".to_string(),
                                decimals: 6,
                                ui_amount: Some(1.0),
                                ui_amount_string: Some("1.0".to_string()),
                            },
                        },
                        account_type: "account".to_string(),
                    },
                    program: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                    space: 165,
                },
                executable: false,
                lamports: 2039280,
                owner: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                rent_epoch: Some(361),
            },
        };

        let json = serde_json::to_string(&token_account_info).unwrap();
        let deserialized: TokenAccountInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.pubkey, "TokenAccountAddress123");
        assert_eq!(
            deserialized.account.data.parsed.info.mint,
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
        );
        assert_eq!(
            deserialized.account.data.parsed.info.token_amount.amount,
            "1000000"
        );
    }

    #[test]
    fn test_token_account_clone() {
        let token_amount = TokenAmount {
            amount: "500000".to_string(),
            decimals: 6,
            ui_amount: Some(0.5),
            ui_amount_string: Some("0.5".to_string()),
        };

        let cloned = token_amount.clone();
        assert_eq!(token_amount.amount, cloned.amount);
        assert_eq!(token_amount.decimals, cloned.decimals);
    }

    #[test]
    fn test_native_token_flag() {
        let mut info = TokenAccountParsedInfo {
            is_native: true,
            mint: "So11111111111111111111111111111111111111112".to_string(),
            owner: "OwnerAddress123".to_string(),
            state: "initialized".to_string(),
            token_amount: TokenAmount {
                amount: "1000000000".to_string(),
                decimals: 9,
                ui_amount: Some(1.0),
                ui_amount_string: Some("1.0".to_string()),
            },
        };

        assert_eq!(info.is_native, true);
        assert_eq!(info.mint, "So11111111111111111111111111111111111111112");

        info.is_native = false;
        assert_eq!(info.is_native, false);
    }
}
