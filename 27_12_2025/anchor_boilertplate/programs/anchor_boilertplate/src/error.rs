use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("The provided math operation resulted in an overflow.")]
    MathOverflow,
    #[msg("Invalid account owner or unauthorized access.")]
    Unauthorized,
}