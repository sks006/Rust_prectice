use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

#[derive(Accounts)]
pub struct Deposit<'info> {
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
        init_if_needed,
        payer = user,
        space = 8 + UserObligation::SIZE,
        seeds = [b"obligation", user.key().as_ref()],
        bump
    )]
    pub obligation: Account<'info, UserObligation>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // Transfer tokens from user to pool vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.pool_vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );

    token::transfer(cpi_ctx, amount)?;

    // Initialize or update obligation
    let obligation = &mut ctx.accounts.obligation;
    
    // Check if obligation is being initialized
    if obligation.owner == Pubkey::default() {
        obligation.owner = ctx.accounts.user.key();
        obligation.deposited_amount = amount;
        obligation.bump = *ctx.bumps.get("obligation").unwrap();
    } else {
        // Update existing obligation
        obligation.deposited_amount = obligation
            .deposited_amount
            .checked_add(amount)
            .ok_or(error::ErrorCode::ArithmeticOverflow)?;
    }

    msg!(
        "User {} deposited {} tokens",
        ctx.accounts.user.key(),
        amount
    );

    Ok(())
}