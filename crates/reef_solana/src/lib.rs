pub mod models;
pub mod provider;
pub mod pubkey;
pub mod rpc;
pub mod utils;

// Solana constants
pub const SOL_DECIMALS: u8 = 9;
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

// Popular token addresses for reference
pub const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
pub const USDT_MINT: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";

// Re-export for convenience
pub use models::*;
pub use pubkey::Pubkey;
pub use rpc::SolanaClient;
