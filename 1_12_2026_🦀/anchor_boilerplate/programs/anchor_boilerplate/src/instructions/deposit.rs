use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use crate::state::UserObligation;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool_vault", mint.key().as_ref()],  // Added mint key
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserObligation::SIZE,
        seeds = [b"obligation", user.key().as_ref()],
        bump
    )]
    pub obligation: Account<'info, UserObligation>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // Transfer logic...
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    token::transfer(cpi_ctx, amount)?;

    let obligation = &mut ctx.accounts.obligation;
    if obligation.owner == Pubkey::default() {
        obligation.owner = ctx.accounts.user.key();
        obligation.bump = ctx.bumps.obligation;
    }
    
    obligation.deposited = obligation.deposited.checked_add(amount).unwrap();
    Ok(())
}