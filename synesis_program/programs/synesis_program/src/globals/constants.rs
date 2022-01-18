#[cfg(not(feature = "local-testing"))]
pub mod constants {
    pub const KANON_STATE_ACCOUNT_PUBKEY: &str = "kngBcrFsdmLQd3JRfdnDGcXaMKsMi8QJtoedEzXCBLa";
    pub const KANON_ADMIN_WALLET_PUBKEY: &str = "JDS6H6dAKEgpwKZ8bXwj9HTGoNWjxrk2XrnovgLaBAcA";
}

#[cfg(feature = "local-testing")]
pub mod constants {
    pub const KANON_STATE_ACCOUNT_PUBKEY: &str = "kngBcrFsdmLQd3JRfdnDGcXaMKsMi8QJtoedEzXCBLa";
    pub const KANON_ADMIN_WALLET_PUBKEY: &str = "JDS6H6dAKEgpwKZ8bXwj9HTGoNWjxrk2XrnovgLaBAcA";
}

pub const KANON_GLOBAL_SEED: &[u8] = b"SYNESIS_KANON";
pub const KANON_GLOBAL_SEASON_SEED: &[u8] = b"SEASON";

pub const COLLECTION_STATE_PREFIX: &[u8] = b"collection_state";
pub const COLLECTION_AUTHORITY_PREFIX: &[u8] = b"collection_authority";
pub const USER_MINT_RESERVE_STATE_PREFIX: &[u8] = b"user_mint_reserve_state";
pub const ADMIN_TREASURY_ACCOUNT_PREFIX: &[u8] = b"admin_treasury_account";

pub const NFT_MINT_COST: u64 = 2000000000;
