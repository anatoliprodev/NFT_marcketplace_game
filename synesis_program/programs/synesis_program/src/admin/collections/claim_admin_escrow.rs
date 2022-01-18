use crate::globals::constants::constants::*;
use crate::globals::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
////////////////////////////////
/// Withdraw funds to admin wallet
/// param:
/// - amount
/// action:
/// Withdraw funds to admin wallet
////////////////////////////////

#[derive(Accounts)]
#[instruction(args: ClaimAdminEscrowArgs)]
pub struct ClaimAdminEscrow<'info> {
    #[account(
        address = KANON_STATE_ACCOUNT_PUBKEY.parse::<Pubkey>().unwrap() @ ErrorCode::InvalidGlobalAccount,
        has_one = admin @ ErrorCode::AccessDenied,
    )]
    pub global_state: Box<Account<'info, GlobalAccount>>,

    #[account(
        mut,
        seeds = [
            global_state.key().as_ref(),
            ADMIN_TREASURY_ACCOUNT_PREFIX.as_ref(),
            KANON_GLOBAL_SEED.as_ref(),
        ],
        bump = global_state.admin_treasury_account_bump,
    )]
    pub admin_treasury_account: AccountInfo<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub receiver_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Processor<ClaimAdminEscrowArgs> for ClaimAdminEscrow<'info> {
    fn process(&mut self, args: ClaimAdminEscrowArgs) -> ProgramResult {
        let global_state_key = self.global_state.key();
        let authority_seeds = [
            global_state_key.as_ref(),
            ADMIN_TREASURY_ACCOUNT_PREFIX.as_ref(),
            KANON_GLOBAL_SEED.as_ref(),
            &[self.global_state.admin_treasury_account_bump],
        ];

        invoke_signed(
            &system_instruction::transfer(
                &self.admin_treasury_account.key,
                &self.admin.to_account_info().key,
                args.amount,
            ),
            &[
                self.admin_treasury_account.clone(),
                self.admin.to_account_info(),
                self.system_program.to_account_info(),
            ],
            &[&authority_seeds[..]],
        )?;
        Ok(())
    }
}

////////////////////////////////
/// Args
////////////////////////////////

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ClaimAdminEscrowArgs {
    pub amount: u64,
}
