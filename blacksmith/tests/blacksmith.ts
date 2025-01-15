import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Blacksmith } from "../target/types/blacksmith";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";

describe("blacksmith", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Blacksmith as Program<Blacksmith>;

  let [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("owner_pda")],
    program.programId
  );

  // it("Create Asset", async () => {

  //   let createAssetArgs = {
  //     name: 'My Asset',
  //     uri: 'https://example.com/my-asset.json',
  //   };

  //   let [pdaUser] = PublicKey.findProgramAddressSync(
  //     [Buffer.from("user_pda")],
  //     program.programId
  //   );

  //   console.log(pda, pdaUser);
  
  //   let asset = Keypair.generate();
  //   const createAssetTx = await program.methods.requestItem(true)
  //     .accountsPartial({
  //       asset: asset.publicKey,
  //       signer: anchor.Wallet.local().publicKey,
  //       systemProgram: SystemProgram.programId,
  //       coreProgram: MPL_CORE_PROGRAM_ID,
  //       ownerPda: pda,
  //       user: pdaUser
  //     })
  //     .signers([asset, anchor.Wallet.local().payer])
  //     .rpc();
  
  //   console.log(createAssetTx);
  // });

  // it("Transfer NFT", async () => {
  //   let asset = new PublicKey("2T4frzu4pic9siNWJ5aTDTUsmCXV8goS1yNFuxR73x6r")
  //   const createAssetTx = await program.methods.transferNft()
  //     .accountsPartial({
  //       asset: asset,
  //       signer: anchor.Wallet.local().publicKey,
  //       newOwner: new PublicKey("AfYgWMf8enCy81Xni5sKBui2EddZSMuJjjTFgnkCmUpQ"),
  //       coreProgram: MPL_CORE_PROGRAM_ID
  //     })
  //     .signers([anchor.Wallet.local().payer])
  //     .rpc();
  
  //   console.log(createAssetTx);
  // })

  // it("Upgrade NFT", async () => {
  //   let asset = new PublicKey("DLzTay33woq4qxHi3eBHSE35aU6m2sMQH9SJxbYa44pZ")
  //   const createAssetTx = await program.methods.upgradeNft()
  //     .accountsPartial({
  //       asset: asset,
  //       payer: anchor.Wallet.local().publicKey,
  //       authority: pda,
  //       coreProgram: MPL_CORE_PROGRAM_ID,
  //       systemProgram: SystemProgram.programId
  //     })
  //     .signers([anchor.Wallet.local().payer])
  //     .rpc();
  
  //   console.log(createAssetTx);
  // })

  it("SCRAP NFT", async() =>  {
    let asset = new PublicKey("3uPwGVT9d7GaaxpRxKwErx8mZgP9t3fctr2NkxHoTXgQ")
    const scrap = await program.methods.scrapItem()
      .accountsPartial({
        asset: asset,
        payer: anchor.Wallet.local().publicKey,
        coreProgram: MPL_CORE_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      .signers([anchor.Wallet.local().payer])
      .rpc();
  
    console.log(scrap);
  })
});
