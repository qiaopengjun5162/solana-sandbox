import {
    airdropFactory,
    createKeyPairSignerFromBytes,
    createSolanaRpc,
    createSolanaRpcSubscriptions,
    generateKeyPairSigner,
    lamports,
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
import secret from './my-keypair.json';

const LAMPORTS_PER_SOL = BigInt(1_000_000_000);

async function main() {
    // 1 - 建立与 Solana 集群的连接
    const httpProvider = 'http://127.0.0.1:8899';
    const wssProvider = 'ws://127.0.0.1:8900';
    const rpc = createSolanaRpc(httpProvider);
    const rpcSubscriptions = createSolanaRpcSubscriptions(wssProvider);
    console.log(`✅ - 已建立与 ${httpProvider} 的连接`);

    // 2 - 生成签名者
    const user1 = await generateKeyPairSigner();
    console.log(`✅ - 新的 user1 地址已创建：${user1.address}`);
    // const user2 = await createKeyPairSignerFromBytes(new Uint8Array([/* 在这里填写你的私钥字节 */]));
    const user2 = await createKeyPairSignerFromBytes(new Uint8Array(secret));

    console.log(`✅ - 从文件生成 user2 地址：${user2.address}`);

    // 3 - 为账户空投 SOL
    // 使用 RPC 方法
    const tx1 = await rpc.requestAirdrop(
        user1.address,
        lamports(LAMPORTS_PER_SOL),
        { commitment: 'processed' }
    ).send();
    console.log(`✅ - user1 使用 RPC 方法空投 1 SOL`);
    console.log(`✅ - tx1: ${tx1}`);

    // 使用工厂函数
    const airdrop = airdropFactory({ rpc, rpcSubscriptions });
    const tx2 = await airdrop({
        commitment: 'processed',
        lamports: lamports(LAMPORTS_PER_SOL),
        recipientAddress: user2.address
    });
    console.log(`✅ - user2 使用工厂函数空投 1 SOL`);
    console.log(`✅ - tx2: ${tx2}`);

    // 4 - 创建转账交易
    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

    const transactionMessage = pipe(
        createTransactionMessage({ version: 0 }), // 初始化新的交易消息。版本为 0
        tx => setTransactionMessageFeePayer(user1.address, tx), // 设置交易的手续费支付者
        tx => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx), // 设置交易的生命周期 使用最近的区块哈希设置交易的生命周期
        tx => appendTransactionMessageInstruction( // 添加转账指令 将转账指令添加到交易中
            getTransferSolInstruction({
                amount: lamports(LAMPORTS_PER_SOL / BigInt(2)),
                destination: user2.address,
                source: user1,
            }),
            tx
        ),
        // tx => appendTransactionMessageInstruction(
        //     getTransferSolInstruction({
        //         amount: lamports(LAMPORTS_PER_SOL / BigInt(3)),
        //         destination: address('SOME_OTHER_ADDRESS'),
        //         source: user1,
        //     }),
        //     tx
        // )
    );
    // 5 - 签名并发送交易
    const signedTransaction = await signTransactionMessageWithSigners(transactionMessage);
    const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });

    try {
        await sendAndConfirmTransaction(
            signedTransaction,
            { commitment: 'confirmed', skipPreflight: true }
        );
        const signature = getSignatureFromTransaction(signedTransaction);
        console.log('✅ - 转账交易:', signature);
    } catch (e) {
        console.error('转账失败:', e);
    }
}

main();

