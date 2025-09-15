use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultTokenInfo {
    pub value: Option<TokenMintData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenMintData {
    pub data: TokenMintParsedData,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenMintParsedData {
    pub parsed: TokenMintParsed,
    pub program: String,
    pub space: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenMintParsed {
    pub info: TokenMintInfo,
    #[serde(rename = "type")]
    pub mint_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenMintInfo {
    pub decimals: u8,
    #[serde(rename = "freezeAuthority")]
    pub freeze_authority: Option<String>,
    #[serde(rename = "isInitialized")]
    pub is_initialized: bool,
    #[serde(rename = "mintAuthority")]
    pub mint_authority: Option<String>,
    pub supply: String,
}
