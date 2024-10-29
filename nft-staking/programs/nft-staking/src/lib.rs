use anchor_lang::prelude::*;

declare_id!("Dfp5Q4UEhtHADpFchLUySUVuSzWz5STG9qQi5HxZXv3n");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}