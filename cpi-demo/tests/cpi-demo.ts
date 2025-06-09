import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { CpiDemo } from "../target/types/cpi_demo";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

describe("cpi-demo", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.cpiDemo as Program<CpiDemo>;
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  const sender = anchor.web3.Keypair.generate();
  const recipient = anchor.web3.Keypair.generate();
  beforeEach(async () => {
    await getBalances(
      wallet.publicKey,
      sender.publicKey,
      "wallet sender Resulting"
    );
    // Fund accounts
    await airdrop(wallet.publicKey, 5 * LAMPORTS_PER_SOL);
    await airdrop(sender.publicKey, 5 * LAMPORTS_PER_SOL);
    await airdrop(recipient.publicKey, 5 * LAMPORTS_PER_SOL);
  });
  async function airdrop(pubkey: PublicKey, amount: number) {
    const sig = await provider.connection.requestAirdrop(pubkey, amount);
    await confirmTransaction(sig);
  }

  async function confirmTransaction(signature: string) {
    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...latestBlockhash,
    });
  }

  // 1 SOL
  const transferAmount = 1 * LAMPORTS_PER_SOL;

  const [PDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("pda"), wallet.publicKey.toBuffer()],
    program.programId
  );

  async function getBalances(
    payerPubkey: PublicKey,
    recipientPubkey: PublicKey,
    timeframe: string
  ) {
    const payerBalance = await provider.connection.getBalance(payerPubkey);
    const recipientBalance = await provider.connection.getBalance(
      recipientPubkey
    );
    console.log(`${timeframe} balances:`);
    console.log(`   Payer: ${payerBalance / LAMPORTS_PER_SOL}`);
    console.log(`   Recipient: ${recipientBalance / LAMPORTS_PER_SOL}`);
  }

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("SOL Transfer Anchor", async () => {
    const transactionSignature = await program.methods
      .solTransfer(new BN(transferAmount))
      .accounts({
        sender: sender.publicKey,
        recipient: recipient.publicKey,
      })
      .signers([sender])
      .rpc();

    console.log(
      `\nTransaction Signature: https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`
    );
    await getBalances(sender.publicKey, recipient.publicKey, "Resulting");
  });

  it("SOL Transfer Anchor wallet", async () => {
    const transactionSignature = await program.methods
      .solTransfer(new BN(transferAmount))
      .accounts({
        sender: wallet.publicKey,
        recipient: recipient.publicKey,
      })
      .rpc();

    console.log(
      `\nTransaction Signature:` +
        `https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`
    );
    await getBalances(
      wallet.publicKey,
      recipient.publicKey,
      "Transfer wallet Resulting"
    );
  });

  it("SOL Transfer2 Anchor", async () => {
    const transactionSignature = await program.methods
      .solTransfer2(new BN(transferAmount))
      .accounts({
        sender: sender.publicKey,
        recipient: recipient.publicKey,
      })
      .signers([sender])
      .rpc();

    console.log(
      `\nTransaction Signature: https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`
    );
    await getBalances(
      sender.publicKey,
      recipient.publicKey,
      "Transfer2 Resulting"
    );
  });

  it("SOL Transfer3 Anchor", async () => {
    const transactionSignature = await program.methods
      .solTransfer3(new BN(transferAmount))
      .accounts({
        sender: sender.publicKey,
        recipient: recipient.publicKey,
      })
      .signers([sender])
      .rpc();

    console.log(
      `\nTransaction Signature: https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`
    );
    await getBalances(
      sender.publicKey,
      recipient.publicKey,
      "Transfer3 Resulting"
    );
  });

  it("Fund PDA with SOL", async () => {
    const transferInstruction = SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: PDA,
      lamports: transferAmount * 2,
    });

    const transaction = new Transaction().add(transferInstruction);

    const transactionSignature = await sendAndConfirmTransaction(
      connection,
      transaction,
      [wallet.payer] // signer
    );

    console.log(
      `\nFund PDA with SOL Transaction Signature:` +
        `https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`
    );

    await getBalances(wallet.publicKey, PDA, "Fund PDA with SOL Resulting");
  });

  it("SOL Transfer with PDA signer", async () => {
    const transactionSignature = await program.methods
      .solTransfer4(new BN(transferAmount))
      .accounts({
        pdaAccount: PDA,
        recipient: wallet.publicKey,
      })
      .rpc();

    console.log(
      `\nSOL Transfer with PDA signer Transaction Signature: https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`
    );

    await getBalances(
      wallet.publicKey,
      PDA,
      "SOL Transfer with PDA signer Resulting"
    );
  });

   it("SOL Transfer with PDA invoke_signed", async () => {
     const transactionSignature = await program.methods
       .solTransfer4(new BN(transferAmount))
       .accounts({
         pdaAccount: PDA,
         recipient: wallet.publicKey,
       })
       .rpc();

     console.log(
       `\nSOL Transfer with PDA invoke_signed Transaction Signature: https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`
     );

     await getBalances(
       wallet.publicKey,
       PDA,
       "SOL Transfer with PDA invoke_signed Resulting"
     );
   });
});
