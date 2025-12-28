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