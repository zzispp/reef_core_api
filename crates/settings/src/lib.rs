use serde::Deserialize;
use std::{env, path::PathBuf};

use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub server: Server,
    pub log: Log,
    pub redis: Database,
    pub postgres: Database,

    pub api: API,
    pub parser: Parser,
    pub daemon: Daemon,


    pub pricer: Pricer,
    pub charter: Charter,
    pub name: Name,
    pub metrics: Metrics,
    pub chains: Chains,
    pub pusher: Pusher,
    pub alerter: Alerter,
    pub scan: Scan,
    pub nft: NFT,
    pub alchemy: Alchemy,
    pub ankr: Ankr,
    pub trongrid: Trongrid,
    pub assets: Assets,
    pub sentry: Option<Sentry>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Server {
    pub address: String,
    pub port: u16,
    pub rocket_log_color: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Log {
    pub level: String,
}


#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}


#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct KeyPublic {
    pub public: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct KeySecret {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Key {
    pub secret: String,
    pub public: String,
}


#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Pricer {
    pub timer: u64,
    pub outdated: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Charter {
    pub timer: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Name {
    pub ens: URL,
    pub ud: UD,
    pub sns: URL,
    pub ton: URL,
    pub eths: URL,
    pub spaceid: URL,
    pub did: URL,
    pub suins: URL,
    pub aptos: URL,
    pub injective: URL,
    pub icns: URL,
    pub lens: URL,
    pub base: URL,
    pub hyperliquid: URL,
    pub alldomains: URL,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct URL {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct UD {
    pub url: String,
    pub key: KeySecret,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Metrics {
    pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Chains {
    pub solana: Chain,
    pub ethereum: Chain,
    pub smartchain: Chain,
    pub polygon: Chain,
    pub optimism: Chain,
    pub arbitrum: Chain,
    pub base: Chain,
    pub opbnb: Chain,
    pub avalanchec: Chain,
    pub ton: Chain,
    pub cosmos: Chain,
    pub osmosis: Chain,
    pub thorchain: Chain,
    pub celestia: Chain,
    pub tron: Chain,
    pub xrp: Chain,
    pub aptos: Chain,
    pub sui: Chain,
    pub bitcoin: Chain,
    pub bitcoincash: Chain,
    pub litecoin: Chain,
    pub doge: Chain,
    pub fantom: Chain,
    pub gnosis: Chain,
    pub injective: Chain,
    pub sei: Chain,
    pub manta: Chain,
    pub blast: Chain,
    pub noble: Chain,
    pub zksync: Chain,
    pub linea: Chain,
    pub mantle: Chain,
    pub celo: Chain,
    pub near: Chain,
    pub world: Chain,
    pub stellar: Chain,
    pub sonic: Chain,
    pub algorand: Chain,
    pub polkadot: Chain,
    pub cardano: Chain,
    #[serde(rename = "abstract")]
    pub abstract_chain: Chain,
    pub berachain: Chain,
    pub ink: Chain,
    pub unichain: Chain,
    pub hyperliquid: Chain,
    pub hypercore: Chain,
    pub monad: Chain,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Chain {
    pub url: String,
    pub archive_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum ChainURLType {
    Default(String),
    Archive(String),
}

impl ChainURLType {
    pub fn get_url(&self) -> String {
        match self {
            ChainURLType::Default(url) => url.clone(),
            ChainURLType::Archive(url) => url.clone(),
        }
    }
}

impl Chain {
    pub fn get_type(&self) -> ChainURLType {
        if let Some(url) = self.archive_url.clone() {
            ChainURLType::Archive(url)
        } else {
            ChainURLType::Default(self.url.clone())
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Parser {
    pub timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Daemon {
    pub service: String,
    pub search: DaemonSearch,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct API {
    pub service: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Pusher {
    pub url: String,
    pub ios: PusherIOS,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct PusherIOS {
    pub topic: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Scan {
    pub timeout_ms: u64,
    pub hashdit: ScanProvider,
    pub goplus: ScanProvider,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ScanProvider {
    pub url: String,
    pub key: Key,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Alerter {
    pub update_interval_seconds: u64,
    pub rules: AlerterRules,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct AlerterRules {
    pub price_increase_percent: f64,
    pub price_decrease_percent: f64,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let current_dir = env::current_dir().unwrap();
        Self::new_setting_path(current_dir.join("Settings.yaml"))
    }

    pub fn new_setting_path(path: PathBuf) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::from(path))
            .add_source(Environment::with_prefix("").prefix_separator("").separator("_"))
            .build()?;
        s.try_deserialize()
    }
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct NFT {
    pub nftscan: NFTScan,
    pub opensea: OpenSea,
    pub magiceden: MagicEden,
    pub bucket: BucketConfiguration,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Alchemy {
    pub key: KeySecret,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Ankr {
    pub key: KeySecret,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Trongrid {
    pub key: KeySecret,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct NFTScan {
    pub key: KeySecret,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct OpenSea {
    pub key: KeySecret,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct MagicEden {
    pub key: KeySecret,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct BucketConfiguration {
    pub endpoint: String,
    pub region: String,
    pub key: Key,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct DaemonSearch {
    pub assets_update_interval: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Assets {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Sentry {
    pub dsn: String,
    pub sample_rate: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_deserialize() {
        let yaml_content = r#"
redis:
  url: "redis://localhost:6379"
postgres:
  url: "postgres://user:pass@localhost/db"
api:
  service: "api_service"
parser:
  timeout: 30
daemon:
  service: "daemon_service"
  search:
    assets_update_interval: 3600
pricer:
  timer: 60
  outdated: 300
coingecko:
  key:
    secret: "coingecko_secret"
charter:
  timer: 120
name:
  ens:
    url: "https://ens.domains"
  ud:
    url: "https://unstoppabledomains.com"
    key:
      secret: "ud_secret"
  sns:
    url: "https://sns.domains"
  ton:
    url: "https://ton.domains"
  eths:
    url: "https://eths.domains"
  spaceid:
    url: "https://space.id"
  did:
    url: "https://did.domains"
  suins:
    url: "https://suins.domains"
  aptos:
    url: "https://aptos.domains"
  injective:
    url: "https://injective.domains"
  icns:
    url: "https://icns.domains"
  lens:
    url: "https://lens.domains"
  base:
    url: "https://base.domains"
  hyperliquid:
    url: "https://hyperliquid.domains"
  alldomains:
    url: "https://alldomains.com"
metrics:
  path: "/metrics"
chains:
  solana:
    url: "https://api.mainnet-beta.solana.com"
  ethereum:
    url: "https://eth-mainnet.g.alchemy.com/v2/demo"
    archive_url: "https://eth-mainnet.archive.com"
  smartchain:
    url: "https://bsc-dataseed.binance.org"
  polygon:
    url: "https://polygon-rpc.com"
  optimism:
    url: "https://mainnet.optimism.io"
  arbitrum:
    url: "https://arb1.arbitrum.io/rpc"
  base:
    url: "https://mainnet.base.org"
  opbnb:
    url: "https://opbnb-mainnet-rpc.bnbchain.org"
  avalanchec:
    url: "https://api.avax.network/ext/bc/C/rpc"
  ton:
    url: "https://toncenter.com/api/v2/jsonRPC"
  cosmos:
    url: "https://cosmos-rpc.polkachu.com"
  osmosis:
    url: "https://osmosis-rpc.polkachu.com"
  thorchain:
    url: "https://thornode.thorchain.info"
  celestia:
    url: "https://celestia-rpc.polkachu.com"
  tron:
    url: "https://api.trongrid.io"
  xrp:
    url: "https://s1.ripple.com:51234"
  aptos:
    url: "https://fullnode.mainnet.aptoslabs.com/v1"
  sui:
    url: "https://fullnode.mainnet.sui.io"
  bitcoin:
    url: "https://bitcoin-rpc.com"
  bitcoincash:
    url: "https://bitcoincash-rpc.com"
  litecoin:
    url: "https://litecoin-rpc.com"
  doge:
    url: "https://doge-rpc.com"
  fantom:
    url: "https://rpc.ftm.tools"
  gnosis:
    url: "https://rpc.gnosischain.com"
  injective:
    url: "https://injective-rpc.polkachu.com"
  sei:
    url: "https://sei-rpc.polkachu.com"
  manta:
    url: "https://manta-rpc.com"
  blast:
    url: "https://blast-rpc.com"
  noble:
    url: "https://noble-rpc.polkachu.com"
  zksync:
    url: "https://zksync-rpc.com"
  linea:
    url: "https://linea-rpc.com"
  mantle:
    url: "https://mantle-rpc.com"
  celo:
    url: "https://celo-rpc.com"
  near:
    url: "https://near-rpc.com"
  world:
    url: "https://world-rpc.com"
  stellar:
    url: "https://stellar-rpc.com"
  sonic:
    url: "https://sonic-rpc.com"
  algorand:
    url: "https://algorand-rpc.com"
  polkadot:
    url: "https://polkadot-rpc.com"
  cardano:
    url: "https://cardano-rpc.com"
  abstract:
    url: "https://abstract-rpc.com"
  berachain:
    url: "https://berachain-rpc.com"
  ink:
    url: "https://ink-rpc.com"
  unichain:
    url: "https://unichain-rpc.com"
  hyperliquid:
    url: "https://hyperliquid-rpc.com"
  hypercore:
    url: "https://hypercore-rpc.com"
  monad:
    url: "https://monad-rpc.com"
pusher:
  url: "https://pusher.com"
  ios:
    topic: "ios_topic"
alerter:
  update_interval_seconds: 60
  rules:
    price_increase_percent: 10.0
    price_decrease_percent: -10.0
scan:
  timeout_ms: 5000
  hashdit:
    url: "https://hashdit.com"
    key:
      secret: "hashdit_secret"
      public: "hashdit_public"
  goplus:
    url: "https://goplus.com"
    key:
      secret: "goplus_secret"
      public: "goplus_public"
nft:
  nftscan:
    key:
      secret: "nftscan_secret"
  opensea:
    key:
      secret: "opensea_secret"
  magiceden:
    key:
      secret: "magiceden_secret"
  bucket:
    endpoint: "https://s3.amazonaws.com"
    region: "us-east-1"
    key:
      secret: "bucket_secret"
      public: "bucket_public"
    name: "nft-bucket"
    url: "https://nft-bucket.s3.amazonaws.com"
alchemy:
  key:
    secret: "alchemy_secret"
ankr:
  key:
    secret: "ankr_secret"
trongrid:
  key:
    secret: "trongrid_secret"
assets:
  url: "https://assets.example.com"
sentry:
  dsn: "https://sentry.io/dsn"
  sample_rate: 1.0
"#;

        let temp_path = std::env::temp_dir().join("test_settings.yaml");
        std::fs::write(&temp_path, yaml_content).unwrap();
        
        let settings = Settings::new_setting_path(temp_path);
        
        assert!(settings.is_ok());
        let settings = settings.unwrap();
        
        // Test basic fields
        assert_eq!(settings.redis.url, "redis://localhost:6379");
        assert_eq!(settings.postgres.url, "postgres://user:pass@localhost/db");
        assert_eq!(settings.api.service, "api_service");
        assert_eq!(settings.parser.timeout, 30);
        
        // Test chains
        assert_eq!(settings.chains.ethereum.url, "https://eth-mainnet.g.alchemy.com/v2/demo");
        assert_eq!(settings.chains.ethereum.archive_url, Some("https://eth-mainnet.archive.com".to_string()));
        assert_eq!(settings.chains.solana.archive_url, None);
    }

    #[test]
    fn test_chain_get_type() {
        let chain_with_archive = Chain {
            url: "https://mainnet.com".to_string(),
            archive_url: Some("https://archive.com".to_string()),
        };
        
        let chain_without_archive = Chain {
            url: "https://mainnet.com".to_string(),
            archive_url: None,
        };
        
        match chain_with_archive.get_type() {
            ChainURLType::Archive(url) => assert_eq!(url, "https://archive.com"),
            _ => panic!("Expected Archive variant"),
        }
        
        match chain_without_archive.get_type() {
            ChainURLType::Default(url) => assert_eq!(url, "https://mainnet.com"),
            _ => panic!("Expected Default variant"),
        }
    }

    #[test]
    fn test_chain_url_type_get_url() {
        let default_type = ChainURLType::Default("https://default.com".to_string());
        let archive_type = ChainURLType::Archive("https://archive.com".to_string());
        
        assert_eq!(default_type.get_url(), "https://default.com");
        assert_eq!(archive_type.get_url(), "https://archive.com");
    }

    #[test]
    fn test_settings_new_missing_file() {
        use std::path::PathBuf;
        let non_existent_path = PathBuf::from("non_existent_settings.yaml");
        let result = Settings::new_setting_path(non_existent_path);
        assert!(result.is_err());
    }
}
