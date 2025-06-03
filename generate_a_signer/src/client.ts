import {
  airdropFactory,
  generateKeyPairSigner,
  lamports,
  MessageSigner,
  SolanaRpcApi,
  Rpc,
  TransactionSigner,
  RpcSubscriptions,
  SolanaRpcSubscriptionsApi,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
} from "@solana/kit";

export type Client = {
  rpc: Rpc<SolanaRpcApi>;
  rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
  wallet: TransactionSigner & MessageSigner;
};

let client: Client | undefined;
export async function createClient(): Promise<Client> {
  if (!client) {
    // Create RPC objects and airdrop function.
    const rpc = createSolanaRpc("http://127.0.0.1:8899");
    const rpcSubscriptions = createSolanaRpcSubscriptions(
      "ws://127.0.0.1:8900"
    );
    const airdrop = airdropFactory({ rpc, rpcSubscriptions });

    // Create a wallet with lamports.
    const wallet = await generateKeyPairSigner();
    await airdrop({
      recipientAddress: wallet.address,
      lamports: lamports(1_000_000_000n),
      commitment: "confirmed",
    });

    // Store the client.
    client = { rpc, rpcSubscriptions, wallet };
  }
  return client;
}
