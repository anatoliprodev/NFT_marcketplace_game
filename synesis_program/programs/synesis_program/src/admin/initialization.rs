use crate::globals::constants::constants::*;
use crate::globals::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

#[derive(Accounts)]
#[instruction(args: InitializationArgs)]
pub struct Initialize<'info> {
    ////////////////////////////////
    /// create global acct
    /// init collection number
    ////////////////////////////////
    #[account(
        zero,
        // address = KANON_STATE_ACCOUNT_PUBKEY.parse::<Pubkey>().unwrap() @ ErrorCode::InvalidGlobalAccount,
        address = Pubkey::from_str(KANON_STATE_ACCOUNT_PUBKEY).unwrap() @ ErrorCode::InvalidGlobalAccount,
    )]
    pub global_state: Box<Account<'info, GlobalAccount>>,

    #[account(
        // seeds = [
        //     global_state.key().as_ref(),
        //     ADMIN_TREASURY_ACCOUNT_PREFIX.as_ref(),
        //     KANON_GLOBAL_SEED.as_ref(),
        // ],
        // bump = args.treasury_account_bump,
    )]
    pub admin_treasury_account: AccountInfo<'info>,

    pub admin: Signer<'info>,
}

impl<'info> Processor<InitializationArgs> for Initialize<'info> {
    fn process(&mut self, args: InitializationArgs) -> ProgramResult {
        self.global_state.season_number = 0;
        // self.global_state.number_of_premints_per_time = 10;
        self.global_state.admin = self.admin.key();
        self.global_state.admin_treasury_account_address = self.admin_treasury_account.key();
        self.global_state.admin_treasury_account_bump = args.treasury_account_bump;

        Ok(())
    }
}

#[derive(Clone, Copy, Default, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct InitializationArgs {
    pub treasury_account_bump: u8,
}
