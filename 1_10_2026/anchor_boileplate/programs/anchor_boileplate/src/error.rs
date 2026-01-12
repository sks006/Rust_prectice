use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient collateral for this borrow")]
    InsufficientCollateral,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid account data")]
    InvalidAccountData,
}