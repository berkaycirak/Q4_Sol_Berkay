use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::Campaign;

// campaign asset collection will be created (nfts based on the campaign collection will be minted when a user invest money to the campaign fundraising)
// campaign vault will be created to collect funds
// asset vault will be created to store money after production is started. RDDL will transfer money to the asset vault since we will sell the energy to RDDL
// only platform owner can invoke this instruction

#[derive(Accounts)]
pub struct StartCampaign<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub campaign_asset_mpl_collection: Signer<'info>,
    #[account(
        init,
        payer=owner,
        space=Campaign::INIT_SPACE,
        seeds=[b"campaign".as_ref()],
        bump,
    )]
    pub campaign: Box<Account<'info, Campaign>>,
    #[account()]
    pub currency_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer=owner,
        associated_token::mint= currency_mint,
        associated_token::authority = campaign,
    )]
    pub campaign_vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}
