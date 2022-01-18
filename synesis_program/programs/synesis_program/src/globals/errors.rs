use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    // Schedule
    #[msg("Invalid duration.")]
    InvalidDuration,
    // Mint Error
    #[msg("Condition is mismatched to mint.")]
    ConditionIsMismatchedToMint,
    #[msg("Total quantity of premint is out of limit.")]
    OutOfWhitelistUserMaxQuantity,
    #[msg("Not enough balance.")]
    NotEnoughBalanceInUserWallet,
    #[msg("Out of User max quantity limitation.")]
    OutOfUserMaxQuantity,

    // Account Validation
    #[msg("The global account address is wrong.")]
    InvalidGlobalAccount,
    #[msg("Access denied.")]
    AccessDenied,
    #[msg("Season number is wrong.")]
    WrongSeasonNumber,
    #[msg("Condition is mismatched.")]
    ConditionMismatch,
}
