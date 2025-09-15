use crate::{
    metaplex::metadata::Metadata,
    models::{Extension, TokenInfo},
};
use primitives::{Asset, AssetType, Chain};

pub fn map_token_data_metaplex(
    chain: Chain,
    token_address: String,
    token_info: &TokenInfo,
    meta: &Metadata,
) -> Result<Asset, Box<dyn std::error::Error + Send + Sync>> {
    let name = meta.data.name.trim_matches(char::from(0)).to_string();
    let symbol = meta.data.symbol.trim_matches(char::from(0)).to_string();
    let decimals = token_info.decimals;
    let asset_type = if token_info.extensions.is_some() {
        AssetType::SPL2022
    } else {
        AssetType::SPL
    };

    Ok(Asset::new(
        name,
        symbol,
        decimals,
        chain,
        Some(token_address),
        asset_type,
    ))
}

pub fn map_token_data_spl_token_2022(
    chain: Chain,
    token_address: String,
    token_info: &TokenInfo,
) -> Result<Asset, Box<dyn std::error::Error + Send + Sync>> {
    let token_metadata = token_info
        .extensions
        .as_ref()
        .and_then(|extensions| {
            extensions.iter().find_map(|ext| {
                if let Extension::TokenMetadata(token_metadata) = ext {
                    Some(token_metadata.state.clone())
                } else {
                    None
                }
            })
        })
        .ok_or("no token metadata found")?;
    Ok(Asset::new(
        token_metadata.name,
        token_metadata.symbol,
        token_info.decimals,
        chain,
        Some(token_address),
        AssetType::SPL2022,
    ))
}

