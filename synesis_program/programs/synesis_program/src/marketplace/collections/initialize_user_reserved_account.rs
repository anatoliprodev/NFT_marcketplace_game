use anchor_lang::prelude::*;
use crate::globals::*;
use std::mem::size_of;
use crate::globals::constants::constants::*;
#[derive(Accounts)]
#[instruction(args: InitializeUserReservedAccountArgs)]
pub struct InitializeUserReservedAccount<'info> {
    #[account(
        address = KANON_STATE_ACCOUNT_PUBKEY.parse::<Pubkey>().unwrap() @ ErrorCode::InvalidGlobalAccount,
        constraint = global_state.season_number == args.season_number @ ErrorCode::WrongSeasonNumber,
    )]
    pub global_state: Box<Account<'info, GlobalAccount>>,

    #[account(
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

    #[account(
        init,
        payer = user,
        seeds = [ 
            collection_state.key().as_ref(), 
            USER_MINT_RESERVE_STATE_PREFIX.as_ref(), 
            KANON_GLOBAL_SEED.as_ref(), 
            KANON_GLOBAL_SEASON_SEED.as_ref(), 
            &[args.season_number].as_ref(),
            user.key().as_ref(), 
        ],
        bump = args.user_mint_reserve_bump,
        space = 8 + size_of::<UserMintReserveAccount>(),
    )]
    pub user_mint_reserve_state: Box<Account<'info, UserMintReserveAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
     
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Processor<InitializeUserReservedAccountArgs> for InitializeUserReservedAccount<'info> {
    fn process(&mut self, args: InitializeUserReservedAccountArgs) -> ProgramResult {
        self.user_mint_reserve_state.bump = args.user_mint_reserve_bump;
        self.user_mint_reserve_state.freely_minted_amount = 0;
        self.user_mint_reserve_state.preminted_amount = 0;
        self.user_mint_reserve_state.airdropped_amount = 0;

        Ok(())
    }
}



////////////////////////////////
/// Args
////////////////////////////////

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeUserReservedAccountArgs {
    pub season_number: u8,
    pub user_mint_reserve_bump: u8,
}
