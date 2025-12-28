use anchor_lang::prelude::*;

#[account]
pub struct UserVault {
    pub owner: Pubkey,      // 32 bytes
    pub collateral: u64,    // 8 bytes
    pub borrowed: u64,      // 8 bytes
    pub bump: u8,           // 1 byte
}

impl UserVault {
    // RULE: Manual space calculation (8 byte discriminator + fields)
    pub const LEN: usize = 8 + 32 + 8 + 8 + 1;
}