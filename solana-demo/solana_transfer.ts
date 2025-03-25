import {
    createKeyPairSignerFromBytes,
    createSolanaRpc,
    createSolanaRpcSubscriptions,
    lamports,
    getBase58Encoder,
    sendAndConfirmTransactionFactory,
    pipe,
    createTransactionMessage,
    setTransactionMessageFeePayer,
    setTransactionMessageLifetimeUsingBlockhash,
    appendTransactionMessageInstruction,
    signTransactionMessageWithSigners,
    getSignatureFromTransaction,
    address,
} from "@solana/web3.js";
import { getTransferSolInstruction } from "@solana-program/system";
// 导入 dotenv
import dotenv from "dotenv";

// 加载 .env 文件中的变量
dotenv.config();
import fs from "fs";

// 访问环境变量
const encoded_data = process.env.ENCODED_DATA;
const private_key = process.env.PRIVATE_KEY;
const user1 = process.env.SOL_ADDRESS1;
const user2 = process.env.SOL_ADDRESS2;
const httpProvider = process.env.SOL_RPC_URL;
const wssProvider = process.env.WSS_PROVIDER;

console.log(`encoded_data: ${encoded_data}`);
if (
    !private_key ||
    !wssProvider ||
    !encoded_data ||
    !user1 ||
    !user2 ||
    !httpProvider
) {
    console.error("Missing environment variables.");
    process.exit(1);
}

const user1Address = address(user1);
const user2Address = address(user2);

// 1 - 创建一个 Solana RPC 客户端
const rpc = createSolanaRpc(httpProvider);
const rpcSubscriptions = createSolanaRpcSubscriptions(wssProvider);
console.log(`✅ - 已建立与 ${httpProvider} 的连接`);

const LAMPORTS_PER_SOL = BigInt(1_000_000_000);

async function main() {
    const encoded_data = [4, 230, 246];
    // const keypairBytes = JSON.parse(fs.readFileSync("../keys/KeykETTNzif4hHZ8dzqM3xNigyAQ4Z3XXyU9yBbM3y9.json").toString())
    // const signer = await createKeyPairSignerFromBytes(new Uint8Array(keypairBytes as number[]));

    const secretKey = private_key as string;
    const signer = await createKeyPairSignerFromBytes(
        getBase58Encoder().encode(secretKey)
    );

    // const seed = new Uint8Array(encoded_data);
    // const signer = await createKeyPairSignerFromBytes(seed);

    // 创建转账交易
    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

    const transactionMessage = pipe(
        createTransactionMessage({ version: 0 }), // 初始化新的交易消息。版本为 0
        (tx) => setTransactionMessageFeePayer(user1Address, tx), // 设置交易的手续费支付者
        (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx), // 设置交易的生命周期 使用最近的区块哈希设置交易的生命周期
        (tx) =>
            appendTransactionMessageInstruction(
                // 添加转账指令 将转账指令添加到交易中
                getTransferSolInstruction({
                    amount: lamports(LAMPORTS_PER_SOL / BigInt(2)),
                    destination: user2Address,
                    source: signer,
                }),
                tx
            )
    );
    // 5 - 签名并发送交易
    const signedTransaction = await signTransactionMessageWithSigners(
        transactionMessage
    );
    const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
        rpc,
        rpcSubscriptions,
    });

    try {
        await sendAndConfirmTransaction(signedTransaction, {
            commitment: "confirmed",
            skipPreflight: true,
        });
        const signature = getSignatureFromTransaction(signedTransaction);
        console.log("✅ - 转账交易:", signature);
    } catch (e) {
        console.error("转账失败:", e);
    }
}

main();

/**
 * 
Web3_wallet/solana-demo on  master [✘?] is 📦 1.0.0 via ⬢ v22.1.0 via 🅒 base 
➜ ts-node solana_transfer.ts
✅ - 已建立与 https://solana-devnet.g.alchemy.com/v2/YLgbp9I-spejSR_9EHp_-UYDrIYdrwE1 的连接
(node:65790) ExperimentalWarning: The Ed25519 Web Crypto API algorithm is an experimental feature and might change at any time
(Use `node --trace-warnings ...` to show where the warning was created)
(node:65790) [UNDICI-WS] Warning: WebSockets are experimental, expect them to change at any time.
✅ - 转账交易: 5BZMYyU1a7ZHtf6q62nkrvrRfc7ee4pLHyiuUk6sPWHkurpNy4obvD4hGmTwFQpjyhtbstConhXbf4EdUQhba6fu


➜ ts-node solana_transfer.ts
✅ - 已建立与 https://solana-devnet.g.alchemy.com/v2/YLgbp9I-spejSR_9EHp_-UYDrIYdrwE1 的连接
(node:75521) ExperimentalWarning: The Ed25519 Web Crypto API algorithm is an experimental feature and might change at any time
(Use `node --trace-warnings ...` to show where the warning was created)
(node:75521) [UNDICI-WS] Warning: WebSockets are experimental, expect them to change at any time.
✅ - 转账交易: 5TX6QHZz9BoXdAourteREavDM2q6ZuSc7FyAE69KemrjZZrfJqhML4YqgHeayMAGQEDNJ68HwDymo7D1miHZqcQX

// https://github.com/anamansari062/test-2.0/blob/main/src/index.js
 */
