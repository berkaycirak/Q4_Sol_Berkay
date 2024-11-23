use anchor_lang::prelude::*;
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


    )]
    pub campaign: Box<Account<'info, Campaign>>,

    pub campaign_vault: InterfaceAccount<'info, TokenAccount>,
}
