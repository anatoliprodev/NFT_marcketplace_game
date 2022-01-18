use anchor_lang::prelude::*;
// use solana_program::hash::{hashv, Hash};
use crate::globals::*;
use crate::globals::constants::constants::*;
use std::mem::size_of;

////////////////////////////////
/// Create PDA state for NFT
///
/// Create PDA for collection authority
////////////////////////////////

#[derive(Accounts)]
#[instruction(args: CreateSeasonArgs)]
pub struct CreateSeason<'info> {
    #[account(
        mut,
        address = KANON_STATE_ACCOUNT_PUBKEY.parse::<Pubkey>().unwrap() @ ErrorCode::InvalidGlobalAccount,
        constraint = (global_state.season_number + 1) == args.season_number @ ErrorCode::WrongSeasonNumber,
        has_one = admin,
    )]
    pub global_state: Box<Account<'info, GlobalAccount>>,

    #[account(
        init,
        payer = admin,
        seeds = [ 
            global_state.key().as_ref(), 
            COLLECTION_STATE_PREFIX.as_ref(), 
            KANON_GLOBAL_SEED.as_ref(), 
            KANON_GLOBAL_SEASON_SEED.as_ref(), 
            &[args.season_number].as_ref() 
        ],
        bump = args.collection_bump,
        space = 8 + size_of::<CollectionAccount>(),
    )]
    pub collection_state: Box<Account<'info, CollectionAccount>>,

    #[account(
        // seeds = [ 
        //     collection_state.key().as_ref(), 
        //     COLLECTION_AUTHORITY_PREFIX.as_ref(), 
        //     KANON_GLOBAL_SEED.as_ref(), 
        //     KANON_GLOBAL_SEASON_SEED.as_ref(), 
        //     &[args.season_number].as_ref() 
        // ],
        // bump = args.authority_bump,
    )]
    pub collection_authority: AccountInfo<'info>,
    #[account(mut)]
    ///pays rent on the initializing pda accounts
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Processor<CreateSeasonArgs> for CreateSeason<'info> {
    fn process(&mut self, args: CreateSeasonArgs) -> ProgramResult {
        //create season
        // self.collection_state.current_step = enums::SeasonStep::PREPARING;
        self.collection_state.airdropped_amount = 0;
        self.collection_state.sold_art_amount = 0;
        self.collection_state.season_opened_timestamp = 0;
        self.collection_state.collection_merkle_hash = args.collection_merkle_root;
        self.collection_state.whitelist_merkle_hash = args.whitelist_merkle_root;

        self.collection_state.countdown_duration = args.countdown_duration;
        self.collection_state.promos_mint_duration = args.promos_mint_duration;
        self.collection_state.premint_duration = args.premint_duration;
        self.collection_state.premint_wave1_duration = args.premint_wave1_duration;
        self.collection_state.premint_blocking_duration = args.premint_blocking_duration;
        self.collection_state.mint_wave3_duration = args.mint_wave3_duration;

        self.collection_state.art_amount = args.art_amount;
        self.collection_state.promos_reserved_nfts_amount = args.promos_reserved_nfts_amount;
        self.collection_state.premint_wave1_amount = args.premint_wave1_amount;
        self.collection_state.premint_wave2_amount = args.premint_wave2_amount;
        self.collection_state.whitelist_user_max_premint_quantity =
            args.whitelist_user_max_premint_quantity;
        self.collection_state.max_freely_mint_quantity = args.max_freely_mint_quantity;
        self.collection_state.bump = args.collection_bump;

        //nft authority
        self.collection_state.authority_bump = args.authority_bump;

        //increase collection number
        self.global_state.season_number += 1;

        //emit event for create of season
        // emit!(event::SeasonStepEvent {
        //     season_number: self.global_state.season_number,
        //     current_step: SeasonStep::PREPARING,
        // });

        Ok(())
    }
}

////////////////////////////////
/// Args
////////////////////////////////

#[derive(Clone, Copy, Default, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct CreateSeasonArgs {
    pub collection_bump: u8,
    pub authority_bump: u8,
    pub season_number: u8,
    pub collection_merkle_root: [u8; 32],
    pub whitelist_merkle_root: [u8; 32],

    // schedules
    pub countdown_duration: i64, // 7 x 24
    pub promos_mint_duration: i64, // 2 x 24
    pub premint_duration: i64,   // 1 x 24
    pub premint_wave1_duration: i64,
    pub premint_blocking_duration: i64, // 7 x 24
    pub mint_wave3_duration: i64,       // 1 x 24

    // limitations
    pub art_amount: u8, // 10,000
    pub promos_reserved_nfts_amount: u8, // 800 NFTs
    pub premint_wave1_amount: u8,                // 1000 NFTs
    pub premint_wave2_amount: u8,                // 1500 NFTs
    pub whitelist_user_max_premint_quantity: u8, // 2 NFTs
    pub max_freely_mint_quantity: u8,            // 1 NFTs
}
