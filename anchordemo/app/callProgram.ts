import { createSolanaClient, createTransaction, generateKeyPairSigner, getExplorerLink, getSignatureFromTransaction, signTransactionMessageWithSigners, SolanaClusterMoniker } from "gill";
import { loadKeypairSignerFromFile } from "gill/node";

import { getInitializeInstruction } from "./generated/ts/instructions";

(async () => {
    const signer = await loadKeypairSignerFromFile("./keys/AnkpTFgp1wzTCZHU7kxQTsit4zQZuqpY4cDzgS5bQnCc.json");

    const cluster: SolanaClusterMoniker = "devnet";

    const { rpc, sendAndConfirmTransaction } = createSolanaClient({
        urlOrMoniker: cluster,
    });


    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

    const dataAccount = await generateKeyPairSigner();

    let ix = getInitializeInstruction({
        authority: signer,
        dataAccount: dataAccount,
        number: 43,
        text: "Hello World",
        optionalKey: null
    })

    let tx = createTransaction({
        version: "legacy",
        feePayer: signer,
        instructions: [ix],
        latestBlockhash,
    });

    let signedTransaction = await signTransactionMessageWithSigners(tx);

    let signature = getSignatureFromTransaction(signedTransaction);
    console.log("Explorer Link:");
    console.log(getExplorerLink({ cluster, transaction: signature }));

    try {
        await sendAndConfirmTransaction(signedTransaction);
        console.log("Transaction confirmed!", signature);
    } catch (e) {
        console.log("Transaction failed!");
        console.log(e);
    }

})();
