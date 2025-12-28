pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GKRNgpjvn5E1wGgw1YcpPT2Xfn3T928iwzePha7e1u1d");

#[program]
pub mod anchor_boilertplate {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
