use crate::globals::constants::constants::*;
use crate::globals::*;
use anchor_lang::prelude::*;

////////////////////////////////
/// Set the amount of premints per time. default is 10.
/// param:
/// - admin wallet pub key to verify authority of pda
/// - mint_amount
/// action:
/// - verify admin wallet authority
/// - set the amount of mint
////////////////////////////////

// #[derive(Accounts)]
// #[instruction(args: SetPremintAmountPerTimeArgs)]
// pub struct SetPremintAmountPerTime<'info> {
//     #[account(
//         mut,
//         address = KANON_STATE_ACCOUNT_PUBKEY.parse::<Pubkey>().unwrap() @ ErrorCode::InvalidGlobalAccount,
//         constraint = args.premint_amount_per_time > 0,
//         has_one = admin,
//     )]
//     pub global_state: Box<Account<'info, GlobalAccount>>,

//     #[account(mut)]
//     pub admin: Signer<'info>,
// }

// impl<'info> Processor<SetPremintAmountPerTimeArgs> for SetPremintAmountPerTime<'info> {
//     fn process(&mut self, args: SetPremintAmountPerTimeArgs) -> ProgramResult {
//         //set the amount of mint
//         self.global_state.number_of_premints_per_time = args.premint_amount_per_time;

//         Ok(())
//     }
// }

// ////////////////////////////////
// /// Args
// ////////////////////////////////

// #[derive(AnchorSerialize, AnchorDeserialize)]
// pub struct SetPremintAmountPerTimeArgs {
//     pub premint_amount_per_time: u8,
// }
