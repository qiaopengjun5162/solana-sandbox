import {
    getExplorerLink,
    createTransaction,
    createSolanaClient,
    getSignatureFromTransaction,
    signTransactionMessageWithSigners,
    generateKeyPairSigner,
    getTransactionCodec,
    Address,
    createSolanaRpc, createSolanaRpcSubscriptions, devnet
} from "gill";
import { loadKeypairSignerFromFile } from "gill/node";
import {
    getAddMemoInstruction,
    getTokenStandardCodec,
    getTransferSolInstruction,
} from "gill/programs";
// 1. 引入依赖

// 2. 创建 Solana 客户端 Solana 客户端设置
const { rpc, sendAndConfirmTransaction } = createSolanaClient({
    urlOrMoniker:
    "https://devnet-rpc.shyft.to?api_key=kurQ_QmwqzSZR95M"
        // "https://solana-devnet.g.alchemy.com/v2/YLgbp9I-spejSR_9EHp_-UYDrIYdrwE1",
});

// const rpc = createSolanaRpc(devnet("https://rpc.shyft.to?api_key=kurQ_QmwqzSZR95M"));
// const rpcSubscriptions = createSolanaRpcSubscriptions(
//     devnet("wss://api.devnet.solana.com")
// );

// 3. 获取最新区块哈希
const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

// 4. 加载签名者和生成随机接收者
const signer = await loadKeypairSignerFromFile();
console.log(`Signer address: ${signer.address}`);

const randomReceiver = await generateKeyPairSigner();

// 5. 构建交易 交易创建
const tx = createTransaction({
    version: "legacy",
    feePayer: signer,
    instructions: [
        getTransferSolInstruction({
            amount: 1_000_000,
            destination: randomReceiver.address,
            source: signer,
        }),
    ],
    latestBlockhash,
    computeUnitLimit: 450,
    computeUnitPrice: 1_000,
});

// 6. 签名交易
const signedTransaction = await signTransactionMessageWithSigners(tx);

const rawTxBytes = getTransactionCodec().encode(signedTransaction);
console.log("Raw transaction bytes:", rawTxBytes);

// 7. 发送并确认交易
try {
    const signedTx = getSignatureFromTransaction(signedTransaction);
    console.log(
        "Explorer link:",
        getExplorerLink({
            cluster: "devnet",
            transaction: signedTx,
        })
    );

    await sendAndConfirmTransaction(signedTransaction);

    console.log("Transaction confirmed!");
} catch (error) {
    console.error("Unable to send and confirm the transaction.");
    console.error("Error:", error);
}
