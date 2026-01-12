use anchor_lang::prelude::*;

#[account]
pub struct UserObligation {
    pub owner: Pubkey,
    pub deposited: u64,
    pub borrowed: u64,
    pub bump: u8,
}

impl UserObligation {
    pub const SIZE: usize = 32 + 8 + 8 + 1;
}