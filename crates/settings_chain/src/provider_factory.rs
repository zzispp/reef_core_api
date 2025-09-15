use crate::{provider_config::NodeType, ProviderConfig};
use primitives::ChainTraits;
use primitives::{Chain, EVMChain};
use reef_evm::rpc::EthereumClient;
use reef_solana::rpc::SolanaClient;
use settings::{ChainURLType, Settings};

pub struct ProviderFactory;

impl ProviderFactory {
    pub async fn new_providers(settings: &Settings) -> Vec<Box<dyn ChainTraits>> {
        let mut providers = Vec::new();
        for chain in Chain::all() {
            // 尝试创建provider，如果失败则跳过
            match Self::new_from_settings(chain, settings).await {
                Ok(provider) => providers.push(provider),
                Err(e) => {
                    println!(
                        "Warning: Failed to create provider for chain: {:?}, error: {}",
                        chain, e
                    );
                }
            }
        }
        providers
    }

    // 不对外暴露
    async fn new_from_settings(
        chain: Chain,
        settings: &Settings,
    ) -> Result<Box<dyn ChainTraits>, Box<dyn std::error::Error + Send + Sync>> {
        let url_type = Self::url(chain, settings);
        let url = url_type.get_url();
        let node_type = ProviderFactory::get_node_type(url_type.clone());
        // 调用new_provider 创建
        Self::new_provider(ProviderConfig::new(
            chain,
            &url,
            node_type,
            settings.alchemy.key.secret.as_str(),
            settings.ankr.key.secret.as_str(),
            settings.trongrid.key.secret.as_str(),
        ))
        .await
    }

    async fn new_provider(
        config: ProviderConfig,
    ) -> Result<Box<dyn ChainTraits>, Box<dyn std::error::Error + Send + Sync>> {
        match config.chain {
            Chain::Ethereum | Chain::SmartChain | Chain::Polygon => {
                let evm_chain = EVMChain::from_chain(config.chain).unwrap();
                let client = EthereumClient::new(config.url, evm_chain);
                Ok(Box::new(client))
            }
            Chain::Solana => {
                let client = SolanaClient::new(config.url);
                Ok(Box::new(client))
            }
        }
    }

    pub fn url(chain: Chain, settings: &Settings) -> ChainURLType {
        match chain {
            Chain::Ethereum => settings.chains.ethereum.get_type(),
            Chain::SmartChain => settings.chains.smartchain.get_type(),
            Chain::Polygon => settings.chains.polygon.get_type(),
            Chain::Solana => settings.chains.solana.get_type(),
        }
    }

    pub fn get_node_type(url: ChainURLType) -> NodeType {
        match url {
            ChainURLType::Default(_) => NodeType::Default,
            ChainURLType::Archive(_) => NodeType::Archive,
        }
    }
}
