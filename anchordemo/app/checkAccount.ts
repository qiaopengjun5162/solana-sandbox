import { createSolanaClient, address, generateKeyPairSigner, getBase58Codec, getExplorerLink, getSignatureFromTransaction, signTransactionMessageWithSigners, SolanaClusterMoniker } from "gill";
import { loadKeypairSignerFromFile } from "gill/node";

import { getDemoDataAccountCodec } from "./generated/ts/accounts";
import { identifyAnchordemoAccount } from "./generated/ts/programs";

(async () => {

    const cluster: SolanaClusterMoniker = "devnet";

    const { rpc, sendAndConfirmTransaction } = createSolanaClient({
        urlOrMoniker: cluster,
    });

    const account = address("6SWBzQWZndeaCKg3AzbY3zkvapCu9bHFZv12iiRoGvCD");
    const info = await rpc.getAccountInfo(account).send();
    const bytes = getBase58Codec().encode(info.value?.data!);

    console.log("identified as "+identifyAnchordemoAccount(bytes));
    

    const decoded = getDemoDataAccountCodec().decode(bytes);
    console.log(decoded);

   

   

})();
