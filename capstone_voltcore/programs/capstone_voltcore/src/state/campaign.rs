use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Campaign {
    pub owner: Pubkey,
    pub bump: u8,
}
