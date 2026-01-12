use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use crate::state::UserObligation;

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool_vault"],
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"obligation", user.key().as_ref()],
        bump,
        // RULE: Security constraint - ensure borrower owns the obligation
        has_one = owner @ ErrorCode::Unauthorized
    )]
    pub obligation: Account<'info, UserObligation>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Borrow>, amount: u64) -> Result<()> {
    let obligation = &ctx.accounts.obligation;

    // RULE: Health Factor Calculation (Gemini HFT Rule)
    let max_borrow = (obligation.deposited as u128)
        .checked_mul(80).unwrap()
        .checked_div(100).unwrap() as u64;

    if amount > max_borrow {
        return Err(error!(ErrorCode::InsufficientCollateral));
    }

    // PDA Signing Rule
    let seeds = &[b"pool_vault", &[ctx.bumps.vault]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer,
    );

    token::transfer(cpi_ctx, amount)?;
    
    ctx.accounts.obligation.borrowed += amount;
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient collateral")]
    InsufficientCollateral,
    #[msg("Unauthorized")]
    Unauthorized,
}