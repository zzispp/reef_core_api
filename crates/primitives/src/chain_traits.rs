use std::error::Error;

use crate::{
    Asset, AssetBalance, BroadcastOptions, Chain, Transaction, TransactionStateRequest,
    TransactionUpdate,
};
use async_trait::async_trait;

pub trait ChainTraits:
    ChainProvider + ChainBalances + ChainTransactions + ChainState + ChainToken + Send + Sync
{
}

pub trait ChainProvider: Send + Sync {
    fn get_chain(&self) -> Chain;
    fn verify_address(&self, address: String) -> Result<(), Box<dyn Error + Sync + Send>>;
}

#[async_trait]
pub trait ChainBalances: Send + Sync {
    async fn get_balance_coin(
        &self,
        address: String,
    ) -> Result<AssetBalance, Box<dyn Error + Sync + Send>>;
    async fn get_balance_tokens(
        &self,
        address: String,
        token_addresses: Vec<String>,
    ) -> Result<Vec<AssetBalance>, Box<dyn Error + Sync + Send>>;
    async fn get_assets_balances(
        &self,
        _address: String,
    ) -> Result<Vec<AssetBalance>, Box<dyn Error + Send + Sync>> {
        Ok(vec![])
    }
}

#[async_trait]
pub trait ChainTransactions: Send + Sync {
   /*  async fn transaction_broadcast(
        &self,
        data: String,
        options: BroadcastOptions,
    ) -> Result<String, Box<dyn Error + Sync + Send>>;
    async fn get_transaction_status(
        &self,
        request: TransactionStateRequest,
    ) -> Result<TransactionUpdate, Box<dyn Error + Sync + Send>>;
    async fn get_transactions_by_address(
        &self,
        _address: String,
        _limit: Option<usize>,
    ) -> Result<Vec<Transaction>, Box<dyn Error + Sync + Send>> {
        Ok(vec![])
    } */
}

#[async_trait]
pub trait ChainState: Send + Sync {
    // async fn get_chain_id(&self) -> Result<String, Box<dyn Error + Sync + Send>>;
    // async fn get_block_latest_number(&self) -> Result<u64, Box<dyn Error + Sync + Send>>;
}

#[async_trait]
pub trait ChainToken: Send + Sync {
    async fn get_token_data(
        &self,
        _token_address: String,
    ) -> Result<Asset, Box<dyn Error + Sync + Send>> {
        Err("Chain does not support tokens".into())
    }

    fn get_is_token_address(&self, _token_address: &str) -> bool {
        false
    }

    //批量获取代币信息
    async fn get_tokens_data(
        &self,
        _token_ids: Vec<String>,
    ) -> Result<Vec<Asset>, Box<dyn Error + Sync + Send>> {
        Err("Chain does not support tokens".into())
    }
}
