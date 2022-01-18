use anchor_lang::prelude::*;
// use solana_program::hash::{hashv, Hash};
use crate::globals::constants::constants::*;
use crate::globals::*;
////////////////////////////////
/// set timestamp
////////////////////////////////

#[derive(Accounts)]
#[instruction(args: TestAdminSetValuesArgs)]
pub struct TestAdminSetValues<'info> {
    #[account(
        mut,
        address = KANON_STATE_ACCOUNT_PUBKEY.parse::<Pubkey>().unwrap() @ ErrorCode::InvalidGlobalAccount,
        has_one = admin,
    )]
    pub global_state: Box<Account<'info, GlobalAccount>>,

    #[account(
        mut,
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
    // pub system_program: Program<'info, System>,
}

impl<'info> Processor<TestAdminSetValuesArgs> for TestAdminSetValues<'info> {
    fn process(&mut self, args: TestAdminSetValuesArgs) -> ProgramResult {
        self.collection_state.season_opened_timestamp = args.test_open_season_timestamp;
        msg!(
            "Current step is: {}",
            self.collection_state
                .get_current_step(self.clock.unix_timestamp)
        );
        Ok(())
    }
}

////////////////////////////////
/// Args
////////////////////////////////

#[derive(Clone, Copy, Default, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct TestAdminSetValuesArgs {
    pub season_number: u8,
    pub test_open_season_timestamp: i64,
}
