import { createSolanaRpc, createSolanaRpcSubscriptions } from "@solana/kit";
import { Rpc, RpcSubscriptions, SolanaRpcApi, SolanaRpcSubscriptionsApi } from "@solana/kit";
import 'dotenv/config';
// 或
// import dotenv from 'dotenv';
// dotenv.config();

const rpcUrl = process.env.SOLANA_RPC_URL || "http://localhost:8899";

export type Client = {
    rpc: Rpc<SolanaRpcApi>;
    rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
};

let client: Client | undefined;
export function createClient(): Client {
    if (!client) {
        client = {
            // rpc: createSolanaRpc(rpcUrl),
            rpc: createSolanaRpc("http://127.0.0.1:8899"),
            rpcSubscriptions: createSolanaRpcSubscriptions("ws://127.0.0.1:8900"),
        };
    }
    return client;
}