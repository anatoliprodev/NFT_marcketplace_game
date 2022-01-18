use crate::globals::constants::constants::*;
use crate::globals::*;
use anchor_lang::prelude::*;
////////////////////////////////
/// Set season step
/// param:
/// - season_number
/// - admin wallet pub key to verify authority of pda
/// - step
/// action:
/// - verify admin wallet authority
/// - set step, it cant be downgrade.
/// - step cant be PREPARING or COUNTDOWN
/// /// emit event
/// //emit event for start of season
// emit!(event::SeasonStepEvent {
//     season_number: self.global_state.season_number,
//     current_step: self.collection_state.current_step,
// });
////////////////////////////////

// #[derive(Accounts)]
// #[instruction(args: SetStepArgs)]
// pub struct SetStep<'info> {
//     #[account(
//         address = KANON_STATE_ACCOUNT_PUBKEY.parse::<Pubkey>().unwrap() @ ErrorCode::InvalidGlobalAccount,
//         constraint = global_state.season_number == args.season_number,
//         has_one = admin,
//     )]
//     pub global_state: Box<Account<'info, GlobalAccount>>,

//     #[account(
//         mut,
//         seeds = [ 
//             global_state.key().as_ref(), 
//             COLLECTION_STATE_PREFIX.as_ref(), 
//             KANON_GLOBAL_SEED.as_ref(), 
//             KANON_GLOBAL_SEASON_SEED.as_ref(), 
//             &[args.season_number].as_ref() 
//         ],
//         bump = collection_state.bump,
//         constraint = collection_state.current_step < args.collection_step && args.collection_step > SeasonStep::COUNTDOWN
//     )]
//     pub collection_state: Box<Account<'info, CollectionAccount>>,

//     #[account(mut)]
//     pub admin: Signer<'info>,
// }

// impl<'info> Processor<SetStepArgs> for SetStep<'info> {
//     fn process(&mut self, args: SetStepArgs) -> ProgramResult {
//         //set step
//         self.collection_state.current_step = args.collection_step;

//         //emit event
//         // emit!(event::SeasonStepEvent {
//         //     season_number: self.global_state.season_number,
//         //     current_step: self.collection_state.current_step,
//         // });

//         Ok(())
//     }
// }

// ////////////////////////////////
// /// Args
// ////////////////////////////////

// #[derive(Clone, Copy, PartialEq, AnchorSerialize, AnchorDeserialize)]
// pub struct SetStepArgs {
//     pub season_number: u8,
//     pub collection_step: SeasonStep,
// }
