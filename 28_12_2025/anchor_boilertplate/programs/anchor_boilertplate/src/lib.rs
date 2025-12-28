pub mod instructions;
pub mod state;
pub mod error;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("BUHbcM1P1UUa4jwWc6Zsen9GMjeM5zX6tABScpsAV3wV");

#[program]
pub mod anchor_boilertplate {
    use super::*;

    // RULE: This name 'deposit' becomes the method name in TypeScript
    pub fn deposit(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        instructions::initialize::handler(ctx, amount)
    }
}