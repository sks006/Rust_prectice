use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub admin: Pubkey,      // 32 bytes
    pub total_deposits: u64, // 8 bytes
    pub bump: u8,           // 1 byte: Canonical PDA bump
}

impl Vault {
    // DISCRIMINATOR (8) + PUBKEY (32) + U64 (8) + U8 (1)
    pub const MAXIMUM_SIZE: usize = 8 + 32 + 8 + 1;
}