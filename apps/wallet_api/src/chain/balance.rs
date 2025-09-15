use actix_web::{get, web};
use std::str::FromStr;
use tokio::sync::Mutex;

use crate::response::{success_response, ApiResult};
use primitives::Chain;
use settings_chain::ChainProviders;

#[get("/chain/balances/{chain}/{address}")]
pub async fn get_balances(
    path: web::Path<(String, String)>,
    providers: web::Data<Mutex<ChainProviders>>,
) -> ApiResult {
    let (chain_str, address) = path.into_inner();
    let chain = Chain::from_str(&chain_str).map_err(|e| anyhow::anyhow!("Invalid chain: {}", e))?;

    // 获取对应链的 provider
    let providers = providers.lock().await;
    let provider = providers
        .get_provider(chain)
        .map_err(|e| anyhow::anyhow!("Provider not found: {}", e))?;

    // 直接调用 provider 的 get_balance 方法
    let balances = provider
        .get_assets_balances(address)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get balances: {}", e))?;

    Ok(success_response(balances))
}
