use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount, Mint};
use crate::state::UserObligation;
use crate::error::ErrorCode;

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
    pub pool_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"obligation", user.key().as_ref()],
        bump = obligation.bump,
    )]
    pub obligation: Account<'info, UserObligation>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Borrow>, amount: u64) -> Result<()> {
    let obligation = &ctx.accounts.obligation;
    
    // Calculate max borrow (80% LTV)
    let max_borrow = obligation
        .deposited_amount
        .checked_mul(80)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(100)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    require!(
        amount <= max_borrow,
        ErrorCode::InsufficientCollateral
    );

    // Transfer from pool vault to user using PDA as authority
    let seeds = &[
        b"pool_vault",
        &[*ctx.bumps.get("pool_vault").unwrap()]
    ];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.pool_vault.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.pool_vault.to_account_info(),
        },
        signer,
    );

    token::transfer(cpi_ctx, amount)?;

    // Update obligation
    let obligation = &mut ctx.accounts.obligation;
    obligation.borrowed_amount = obligation
        .borrowed_amount
        .checked_add(amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    msg!(
        "User {} borrowed {} tokens",
        ctx.accounts.user.key(),
        amount
    );

    Ok(())
}