import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CapstoneVoltcore } from "../target/types/capstone_voltcore";
import { MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";

describe("capstone_voltcore", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace
		.CapstoneVoltcore as Program<CapstoneVoltcore>;

	it("An asset collection is created for the campaign", async () => {
		const campaignAssetMplCollection = anchor.web3.Keypair.generate();

		// Add your test here.
		const tx = await program.methods
			.createCampaignAsset({
				name: "Test Asset",
				uri: "https://example.com/asset",
			})
			.accountsPartial({
				payer: anchor.getProvider().publicKey,
				campaignAssetMplCollection:
					campaignAssetMplCollection.publicKey,
				updateAuthority: null,
				systemProgram: anchor.web3.SystemProgram.programId,
				mplCoreProgram: MPL_CORE_PROGRAM_ID,
			})
			.signers([campaignAssetMplCollection])
			.rpc();
		console.log("Your transaction signature", tx);
	});
});
