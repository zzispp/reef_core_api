use crate::Chain;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssetBalance {
    pub chain: Chain,
    pub contract_address: Option<String>,
    pub balance: Balance,
    pub is_active: Option<bool>,
}

impl AssetBalance {
    pub fn new(
        chain: Chain,
        contract_address: Option<String>,
        balance: BigUint,
        decimals: u8,
    ) -> Self {
        Self {
            chain,
            contract_address,
            balance: Balance::coin_balance(balance, decimals),
            is_active: None,
        }
    }

    pub fn new_balance(chain: Chain, contract_address: Option<String>, balance: Balance) -> Self {
        Self {
            chain,
            contract_address,
            balance,
            is_active: None,
        }
    }

    pub fn new_token(
        chain: Chain,
        contract_address: Option<String>,
        amount: BigUint,
        decimals: u8,
    ) -> Self {
        Self {
            chain,
            contract_address,
            balance: Balance::token_balance(amount, decimals),
            is_active: Some(true),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub decimals: u8,
    pub amount: String,
    pub ui_amount: Option<f64>,
}

impl Balance {
    pub fn new(amount: String, decimals: u8, ui_amount: Option<f64>) -> Self {
        Self {
            decimals,
            amount,
            ui_amount,
        }
    }

    pub fn coin_balance(available: BigUint, decimals: u8) -> Self {
        let amount_str = available.to_string();
        let ui_amount = if decimals > 0 {
            let divisor = 10_u64.pow(decimals as u32) as f64;
            Some(amount_str.parse::<f64>().unwrap_or(0.0) / divisor)
        } else {
            Some(amount_str.parse::<f64>().unwrap_or(0.0))
        };

        Self {
            decimals,
            amount: amount_str,
            ui_amount,
        }
    }

    pub fn token_balance(amount: BigUint, decimals: u8) -> Self {
        let amount_str = amount.to_string();
        let ui_amount = if decimals > 0 {
            let divisor = 10_u64.pow(decimals as u32) as f64;
            Some(amount_str.parse::<f64>().unwrap_or(0.0) / divisor)
        } else {
            Some(amount_str.parse::<f64>().unwrap_or(0.0))
        };

        Self {
            decimals,
            amount: amount_str,
            ui_amount,
        }
    }
}
