use anchor_lang::prelude::*;
pub mod admin;
pub mod globals;
pub mod marketplace;

use admin::*;
use globals::*;
use marketplace::*;

//declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[cfg(not(feature = "local-testing"))]
declare_id!("Sns8ZPjkAART2Hfmcs4CjwN1RgrdJsZhyEptXkrNoY8");
#[cfg(feature = "local-testing")]
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod synesis_program {
    use super::*;

    /////////////////////////////////////
    ///  admin portal
    /////////////////////////////////////
    pub fn initialize(ctx: Context<Initialize>, args: InitializationArgs) -> ProgramResult {
        ctx.accounts.process(args)
    }

    pub fn create_season(ctx: Context<CreateSeason>, args: CreateSeasonArgs) -> ProgramResult {
        ctx.accounts.process(args)
    }

    pub fn open_season(ctx: Context<OpenSeason>, args: OpenSeasonArgs) -> ProgramResult {
        ctx.accounts.process(args)
    }

    pub fn claim_admin_escrow(
        ctx: Context<ClaimAdminEscrow>,
        args: ClaimAdminEscrowArgs,
    ) -> ProgramResult {
        ctx.accounts.process(args)
    }

    /////////////////////////////////////
    /// marketplace
    /////////////////////////////////////
    pub fn initialize_user_reserved_account(
        ctx: Context<InitializeUserReservedAccount>,
        args: InitializeUserReservedAccountArgs,
    ) -> ProgramResult {
        ctx.accounts.process(args)
    }

    pub fn claim_airdrop_reserved_nfts(
        ctx: Context<ClaimAirdropReservedNfts>,
        args: ClaimAirdropReservedNftsArgs,
    ) -> ProgramResult {
        ctx.accounts.process(args)
    }

    pub fn mint_whitelist_nft_one(
        ctx: Context<MintWhitelistNftOne>,
        args: MintWhitelistNftOneArgs,
    ) -> ProgramResult {
        ctx.accounts.process(args)
    }

    pub fn freely_mint_nft_one(
        ctx: Context<FreelyMintNftOne>,
        args: FreelyMintNftOneArgs,
    ) -> ProgramResult {
        ctx.accounts.process(args)
    }

    // ////////////////////////////////
    // /// For test only
    // ////////////////////////////////

    // pub fn test_admin_set_values(
    //     ctx: Context<TestAdminSetValues>,
    //     args: TestAdminSetValuesArgs,
    // ) -> ProgramResult {
    //     ctx.accounts.process(args)
    // }
    // ////////////////////////////////
}
