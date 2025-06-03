import { address, Address, Lamports, } from "@solana/kit";
import { createClient } from "./client";



async function getBalance(account: Address): Promise<Lamports> {
    const client = createClient();

    const { value: balance } = await client.rpc.getBalance(account).send();
    console.log(`Balance of ${account}: ${balance}`);
    return balance;

}

export default getBalance;