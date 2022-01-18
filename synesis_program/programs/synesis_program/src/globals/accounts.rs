use crate::globals::*;
use anchor_lang::{prelude::*, solana_program};

#[account]
pub struct GlobalAccount {
    pub season_number: u8,
    // pub number_of_premints_per_time: u8,
    pub admin: Pubkey,
    pub admin_treasury_account_address: Pubkey,
    pub admin_treasury_account_bump: u8,
}

#[account]
pub struct CollectionAccount {
    pub bump: u8,
    pub authority_bump: u8,
    // user wallet hash
    pub whitelist_merkle_hash: [u8; 32],
    // NFT word hash
    pub collection_merkle_hash: [u8; 32],
    // NFT mint address hash
    pub nft_merkle_hash: [u8; 32],
    // Airdrop-NFT mint address hash
    pub reserved_nft_merkle_hash: [u8; 32],

    // pub current_step: SeasonStep,
    pub airdropped_amount: u8,
    pub sold_art_amount: u8,
    pub season_opened_timestamp: i64,
    ////////////////////////////////
    /// Schedule season step(hour unit)
    ///
    /// |---------CountDown(7d)--------|----------------Premint(1d)----------------|---BlockingMint(7d)---|---Freely Mint(1d)---|------Publish------|---End Season
    ///       |----Promos Minted(2d)---|-----Wave1(x hr)-----|-----Wave2(y hr)-----|                      |---------Wave3-------|
    ///       |---Quantity Limitation--|-Quantity Limitation-|-Quantity Limitation-|
    ///       |-------------------------------------------------MysteryBox period-----------------------------------------------|--Update Metadata--|
    ///
    ////////////////////////////////
    pub countdown_duration: i64, // 7 x 24
    pub promos_mint_duration: i64, // 2 x 24

    pub premint_duration: i64, // 1 x 24
    pub premint_wave1_duration: i64,

    pub premint_blocking_duration: i64, // 7 x 24
    pub mint_wave3_duration: i64,       // 1 x 24
    ////////////////////////////////
    /// Quantity Limitation
    ///
    ///  |--------------------------------------------------------Art amount(10,000)----------------------------------------------------------|
    ///  |--promos_reserved_nft_amount(800)--|--premint_wave1_amount(1000)--|--premint_wave2_amount(1500)--|------the rest(Freely mint)-------|
    ///
    ////////////////////////////////
    pub art_amount: u8,
    // airdrop nfts
    pub promos_reserved_nfts_amount: u8, // 800 NFTs

    pub premint_wave1_amount: u8,                // 1000 NFTs
    pub premint_wave2_amount: u8,                // 1500 NFTs
    pub whitelist_user_max_premint_quantity: u8, // 2 NFTs
    pub max_freely_mint_quantity: u8,            // 3 NFT
}

impl CollectionAccount {
    pub fn get_current_step(&self, current_timestamp: i64) -> SeasonStep {
        if self.season_opened_timestamp == 0 {
            return SeasonStep::PREPARING;
        }

        let mut duration: f64 =
            (current_timestamp - self.season_opened_timestamp) as f64 / 3600 as f64;
        msg!("Duration is: {}", duration);
        msg!(
            "season_opened_timestamp is: {}",
            self.season_opened_timestamp
        );
        msg!("current_timestamp is: {}", current_timestamp);

        duration -= self.countdown_duration as f64;
        if duration <= 0 as f64 {
            return SeasonStep::COUNTDOWN;
        }

        duration -= self.premint_duration as f64;
        if duration <= 0 as f64 {
            return SeasonStep::PREMINT;
        }

        duration -= self.premint_blocking_duration as f64;
        if duration <= 0 as f64 {
            return SeasonStep::BLOCKING;
        }

        duration -= self.mint_wave3_duration as f64;
        if duration <= 0 as f64 {
            return SeasonStep::FREELYMINT;
        }

        return SeasonStep::PUBLISHED;
    }

    pub fn verify_whitelist(
        &self,
        user_key: Pubkey,
        proof: Vec<[u8; 32]>,
        season_number: u8,
    ) -> bool {
        let merkle_seed = [
            KANON_GLOBAL_SEED,
            b"&",
            KANON_GLOBAL_SEASON_SEED,
            b"&",
            &[season_number].as_ref(),
        ]
        .concat();

        let node = solana_program::keccak::hashv(&[&merkle_seed.as_ref(), &user_key.to_bytes()]);
        merkle_tree_verify(proof, self.whitelist_merkle_hash, node.0)
    }

    pub fn verify_reserved_nft_whitelist(
        &self,
        user_key: Pubkey,
        nft_mint_address: Pubkey,
        proof: Vec<[u8; 32]>,
        season_number: u8,
    ) -> bool {
        let merkle_seed = [
            KANON_GLOBAL_SEED,
            b"&",
            KANON_GLOBAL_SEASON_SEED,
            b"&",
            &[season_number].as_ref(),
        ]
        .concat();

        let node = solana_program::keccak::hashv(&[
            &merkle_seed.as_ref(),
            &user_key.to_bytes(),
            &nft_mint_address.to_bytes(),
        ]);
        merkle_tree_verify(proof, self.reserved_nft_merkle_hash, node.0)
    }
}

#[account]
pub struct UserMintReserveAccount {
    pub bump: u8,
    pub preminted_amount: u8,
    pub freely_minted_amount: u8,
    pub airdropped_amount: u8,
}
