use anchor_lang::prelude::*;
// use solana_program::hash::{hashv, Hash};
use crate::globals::constants::constants::*;
use crate::globals::*;
////////////////////////////////
/// Set step to COUNTDOWN
/// Record start timestamp
/// Put nft merkle root
////////////////////////////////

#[derive(Accounts)]
#[instruction(args: OpenSeasonArgs)]
pub struct OpenSeason<'info> {
    #[account(
        address = KANON_STATE_ACCOUNT_PUBKEY.parse::<Pubkey>().unwrap() @ ErrorCode::InvalidGlobalAccount,
        constraint = global_state.season_number == args.season_number @ ErrorCode::WrongSeasonNumber,
        has_one = admin,
    )]
    pub global_state: Box<Account<'info, GlobalAccount>>,

    #[account(
        mut,
        constraint = collection_state.get_current_step(clock.unix_timestamp) == SeasonStep::PREPARING @ ErrorCode::ConditionMismatch,
        seeds = [
            global_state.key().as_ref(),
            COLLECTION_STATE_PREFIX.as_ref(),
            KANON_GLOBAL_SEED.as_ref(),
            KANON_GLOBAL_SEASON_SEED.as_ref(),
            &[args.season_number].as_ref()
        ],
        bump = collection_state.bump,
    )]
    pub collection_state: Box<Account<'info, CollectionAccount>>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub(crate) clock: Sysvar<'info, Clock>,
    // pub system_program: Program<'info, System>,  ?????
}

impl<'info> Processor<OpenSeasonArgs> for OpenSeason<'info> {
    fn process(&mut self, args: OpenSeasonArgs) -> ProgramResult {
        //open season
        self.collection_state.season_opened_timestamp = self.clock.unix_timestamp;
        self.collection_state.reserved_nft_merkle_hash = args.reserved_nft_merkle_root;
        self.collection_state.nft_merkle_hash = args.nft_merkle_root;

        //emit event for open of season
        emit!(event::SeasonStepEvent {
            season_number: self.global_state.season_number,
            current_step: SeasonStep::COUNTDOWN,
        });

        Ok(())
    }
}

////////////////////////////////
/// Args
////////////////////////////////

#[derive(Clone, Copy, Default, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct OpenSeasonArgs {
    pub season_number: u8,
    pub reserved_nft_merkle_root: [u8; 32],
    pub nft_merkle_root: [u8; 32],
}
