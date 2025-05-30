import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { HelloAnchor } from "../target/types/hello_anchor";
import { Keypair } from "@solana/web3.js";
import assert  from "assert";

describe("hello_anchor", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.helloAnchor as Program<HelloAnchor>;

  it("Is initialized!", async () => {
    const newAccountKp = new Keypair();

    const data = new BN(42);
    const tx = await program.methods.initialize(data)
      .accounts({
        newAccount: newAccountKp.publicKey,
        signer: wallet.publicKey,
      })
      .signers([newAccountKp])
      .rpc();
    console.log("Your transaction signature", tx);

    const newAccount = await program.account.newAccount.fetch(newAccountKp.publicKey);
    assert.ok(newAccount.data.eq(data));
    console.log("New account data:", newAccount.data.toString());
    assert(data.eq(newAccount.data));
  });
});
