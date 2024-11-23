pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2Fbhe8DXB7FGTqfBvuzn7FDAUQHuy5Lae8XBLthaYxbD");

#[program]
pub mod capstone_voltcore {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
