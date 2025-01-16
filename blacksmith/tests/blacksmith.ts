import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Blacksmith } from "../target/types/blacksmith";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";

describe("blacksmith", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Blacksmith as Program<Blacksmith>;

  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("owner_pda")],
    program.programId
  );

  const [pdaUser] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_pda")],
    program.programId
  );

  const asset = Keypair.generate();
  console.log(asset.publicKey)
  it("Request Item", async () => {
    const createAssetTx = await program.methods
      .requestItem(true)
      .accountsPartial({
        asset: asset.publicKey,
        signer: anchor.Wallet.local().publicKey,
        systemProgram: SystemProgram.programId,
        coreProgram: MPL_CORE_PROGRAM_ID,
        ownerPda: pda,
        user: pdaUser,
      })
      .signers([asset, anchor.Wallet.local().payer])
      .rpc();

    console.log(createAssetTx);
  });

  it("Upgrade Item", async () => {
    const upgradeNftTx = await program.methods
      .upgradeNft()
      .accountsPartial({
        asset: asset.publicKey,
        payer: anchor.Wallet.local().publicKey,
        authority: pda,
        coreProgram: MPL_CORE_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([anchor.Wallet.local().payer])
      .rpc();

    console.log(upgradeNftTx);
  });

  it("Scrap NFT", async () => {
    const scrapNftTx = await program.methods
      .scrapItem()
      .accountsPartial({
        asset: asset.publicKey,
        payer: anchor.Wallet.local().publicKey,
        authority: pda,
        coreProgram: MPL_CORE_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([anchor.Wallet.local().payer])
      .rpc();

    console.log(scrapNftTx);
  });

  /*
  it("Transfer NFT", async () => {
    const asset = new PublicKey("2T4frzu4pic9siNWJ5aTDTUsmCXV8goS1yNFuxR73x6r");
    const transferNftTx = await program.methods.transferNft()
      .accountsPartial({
        asset: asset,
        signer: anchor.Wallet.local().publicKey,
        newOwner: new PublicKey("AfYgWMf8enCy81Xni5sKBui2EddZSMuJjjTFgnkCmUpQ"),
        coreProgram: MPL_CORE_PROGRAM_ID,
      })
      .signers([anchor.Wallet.local().payer])
      .rpc();

    console.log(transferNftTx);
  });
  */
});
