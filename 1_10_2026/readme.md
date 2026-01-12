# Week 6 Goal: "Account Context"
// Type this 50 times/day from blank file
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

#[derive(Accounts)]
pub struct Deposit<'info> {
    // RULE 1: User must sign (Authorize the transaction)
    #[account(mut)]
    pub user: Signer<'info>,

    // RULE 2: User's token account changes (tokens leave it)
    #[account(
        mut,
        token::mint = mint,
        token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    // RULE 3: Program's vault changes (tokens enter it)
    #[account(
        mut,
        seeds = [b"pool_vault"],
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,

    // RULE 4: User's obligation record (initialized if needed)
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + 8 + 1, // 57 bytes
        seeds = [b"obligation", user.key().as_ref()],
        bump
    )]
    pub obligation: Account<'info, UserObligation>,

    // RULE 5: Required accounts for token operations
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

// RULE 6: Always include the state struct
#[account]
pub struct UserObligation {
    pub owner: Pubkey,           // 32 bytes
    pub deposited: u64,          // 8 bytes
    pub borrowed: u64,           // 8 bytes
    pub bump: u8,                // 1 byte
}