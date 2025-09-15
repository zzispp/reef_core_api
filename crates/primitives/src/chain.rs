use std::fmt;

use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};
use strum_macros::{AsRefStr, EnumString};
use typeshare::typeshare;

#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    EnumIter,
    AsRefStr,
    EnumString,
    PartialEq,
    Ord,
    PartialOrd,
    Eq,
    Hash,
)]
#[typeshare(swift = "Equatable, CaseIterable, Sendable, Hashable")]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Chain {
    Solana,
    Ethereum,
    BinanceSmartChain,
    Polygon,
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Chain {
    pub fn all() -> Vec<Self> {
        Self::iter().collect()
    }
}

#[typeshare]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum EVMChain {
    Ethereum,
    BinanceSmartChain,
    Polygon,
}

impl EVMChain {
    pub fn from_chain(chain: Chain) -> Option<Self> {
        match chain {
            Chain::Ethereum => Some(Self::Ethereum),
            Chain::BinanceSmartChain => Some(Self::BinanceSmartChain),
            Chain::Polygon => Some(Self::Polygon),
            _ => None,
        }
    }

    pub fn to_chain(self) -> Chain {
        match self {
            Self::Ethereum => Chain::Ethereum,
            Self::BinanceSmartChain => Chain::BinanceSmartChain,
            Self::Polygon => Chain::Polygon,
        }
    }

    pub fn chain_id(&self) -> u64 {
        match self {
            Self::Ethereum => 1,
            Self::BinanceSmartChain => 56,
            Self::Polygon => 137,
        }
    }

    pub fn native_symbol(&self) -> &'static str {
        match self {
            Self::Ethereum => "ETH",
            Self::BinanceSmartChain => "BNB",
            Self::Polygon => "MATIC",
        }
    }

    pub fn native_decimals(&self) -> u8 {
        18
    }

    pub fn all() -> Vec<Self> {
        Self::iter().collect()
    }
}
