import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CreateMintAccount } from "../target/types/create_mint_account";
import { TOKEN_2022_PROGRAM_ID, getMint } from "@solana/spl-token";

describe("create-mint-account", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .createMintAccount as Program<CreateMintAccount>;
  const mint = anchor.web3.Keypair.generate();
  const [mint2, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId,
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Create mint", async () => {
    const create_mint_tx = await program.methods
      .createMint()
      .accounts({
        mint: mint.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([mint])
      .rpc({ commitment: "confirmed" });
    console.log("Your transaction signature", create_mint_tx);

    const mintAccount = await getMint(
      program.provider.connection,
      mint.publicKey,
      "confirmed",
      TOKEN_2022_PROGRAM_ID
    );

    console.log("Mint Account", mintAccount);
  });

  it("Create Mint By PDA!", async () => {
    const tx = await program.methods
      .createMint2()
      .accounts({
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc({ commitment: "confirmed" });
    console.log("Your createMint transaction signature", tx);

    const mintAccount = await getMint(
      program.provider.connection,
      mint2,
      "confirmed",
      TOKEN_2022_PROGRAM_ID,
    );

    console.log("Mint Account", mintAccount);
  });
});
