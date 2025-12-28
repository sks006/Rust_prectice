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