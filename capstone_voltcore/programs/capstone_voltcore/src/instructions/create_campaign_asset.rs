use anchor_lang::prelude::*;
use mpl_core::{instructions::CreateCollectionV2CpiBuilder, ID as MPL_CORE_ID};

// platform owner can invoke this instruction to create a campaign asset collection
// this instruction will create a new asset collection and a new asset vault

#[derive(Accounts)]
pub struct CreateCampaignAsset<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub campaign_asset_mpl_collection: Signer<'info>, // a collection representing the campaign asset will be created and owned by the mpl-core program
    /// CHECK: this account will be checked by the mpl_core program
    pub update_authority: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
    ///CHECK: this account is checked by the address constraint
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateCampaignAssetArgs {
    pub name: String,
    pub uri: String,
}

impl<'info> CreateCampaignAsset<'info> {
    pub fn process(&mut self, data: CreateCampaignAssetArgs) -> Result<()> {
        let update_authority = match self.update_authority.as_ref() {
            Some(update_authority) => Some(update_authority.to_account_info()),
            None => None,
        };

        CreateCollectionV2CpiBuilder::new(self.mpl_core_program.to_account_info().as_ref())
            .collection(
                self.campaign_asset_mpl_collection
                    .to_account_info()
                    .as_ref(),
            )
            .payer(self.payer.to_account_info().as_ref())
            .update_authority(update_authority.as_ref())
            .system_program(self.system_program.to_account_info().as_ref())
            .name(data.name)
            .uri(data.uri)
            .invoke()?;

        Ok(())
    }
}
