import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Arena } from "../target/types/arena";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("arena", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Arena as Program<Arena>;

  it("Fight!", async () => {
    const tx = await program.methods
    .fight()
    .accountsPartial({
      payer: anchor.Wallet.local().publicKey,
      chestplate: null,
      gloves: null,
      boots: null,
      sword: null,
      helmet: new PublicKey("Bp2wZcgdw8BWkuiEpw6ai4HRthTvrgFB3HtVcTitAoY9"),
      neck: null,
      ring: null,
    })
    .signers([anchor.Wallet.local().payer])
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
