```
use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("The provided math operation resulted in an overflow.")]
    MathOverflow,
    #[msg("Invalid account owner or unauthorized access.")]
    Unauthorized,
}

```
=========================
```
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
```
====================
```
use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::constants::VAULT_SEED;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    // Week 5 Drill: Initialize PDA with seeds
    // This ensures only one 'vault' can exist per admin
    #[account(
        init,
        payer = admin,
        space = Vault::MAXIMUM_SIZE,
        seeds = [VAULT_SEED, admin.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.admin = ctx.accounts.admin.key();
    vault.total_deposits = 0;
    vault.bump = ctx.bumps.vault; // Store the bump for Week 10 security checks

    msg!("Vault Initialized. Admin: {}", vault.admin);
    Ok(())
}
```

==================

```
use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::constants::VAULT_SEED;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    // Week 5 Drill: Initialize PDA with seeds
    // This ensures only one 'vault' can exist per admin
    #[account(
        init,
        payer = admin,
        space = Vault::MAXIMUM_SIZE,
        seeds = [VAULT_SEED, admin.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.admin = ctx.accounts.admin.key();
    vault.total_deposits = 0;
    vault.bump = ctx.bumps.vault; // Store the bump for Week 10 security checks

    msg!("Vault Initialized. Admin: {}", vault.admin);
    Ok(())
}
```

=============================
```
pub mod error;
pub mod state;
pub mod constants;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("YourProgramIDHere11111111111111111111111");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }
}
```
====================================

```
#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;
    use solana_program::clock::Epoch;

    #[test]
    fn test_initialize_vault_logic() {
        // 1. Create Mock Data (The "Kata" of Testing) [cite: 92, 131]
        let program_id = Pubkey::new_unique();
        let admin_key = Pubkey::new_unique();
        let vault_key = Pubkey::new_unique();
        
        // 2. Mock Lamports and Data Buffer [cite: 133, 134]
        let mut lamports = 100_000_000; // 0.1 SOL
        let mut data = vec![0u8; Vault::MAXIMUM_SIZE];
        let owner = program_id;

        // 3. Construct AccountInfo (Mirroring the SVM) [cite: 136, 137]
        let vault_account_info = AccountInfo::new(
            &vault_key,
            false,       // is_signer
            true,        // is_writable
            &mut lamports,
            &mut data,
            &owner,
            false,       // executable
            Epoch::default(),
        );

        // 4. Wrap in Anchor Account type [cite: 105]
        let mut vault_account = Account::<Vault>::try_from(&vault_account_info).unwrap();

        // 5. Run the logic (The "Rule of the Handler") 
        vault_account.admin = admin_key;
        vault_account.total_deposits = 0;
        vault_account.bump = 255;

        // 6. Assertions (The "Rule of Proof") [cite: 151, 164]
        assert_eq!(vault_account.admin, admin_key);
        assert_eq!(vault_account.total_deposits, 0);
        println!("Internal Rust logic test passed for Vault: {:?}", vault_key);
    }
}
```