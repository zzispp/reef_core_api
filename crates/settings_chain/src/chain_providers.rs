use std::collections::HashMap;
use std::error::Error;

use primitives::Chain;
use primitives::ChainTraits;
use settings::Settings;

use crate::ProviderFactory;

pub struct ChainProviders {
    provider_map: HashMap<Chain, Box<dyn ChainTraits>>,
}

impl ChainProviders {
    pub fn new(providers: Vec<Box<dyn ChainTraits>>) -> Self {
        let mut provider_map = HashMap::new();

        // 构建chain到provider的映射
        for provider in providers {
            let chain = provider.get_chain();
            provider_map.insert(chain, provider);
        }

        Self { provider_map }
    }

    pub async fn from_settings(settings: &Settings) -> Self {
        Self::new(ProviderFactory::new_providers(settings).await)
    }

    pub fn get_provider(
        &self,
        chain: Chain,
    ) -> Result<&dyn ChainTraits, Box<dyn Error + Send + Sync>> {
        // o1 查找
        self.provider_map
            .get(&chain)
            .map(|p| p.as_ref())
            .ok_or_else(|| format!("No provider found for chain: {:?}", chain).into())
    }
}
