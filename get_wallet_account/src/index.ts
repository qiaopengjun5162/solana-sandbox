import GetBalance from './get_balance';
import { address } from "@solana/kit";


const main = async () => {
    const account = address("6MZDRo5v8K2NfdohdD76QNpSgk3GH3Aup53BeMaRAEpd");
    const balance = await GetBalance(account);
    console.log(`Balance: ${balance} lamports.`);
}

main().catch(err => {
    console.error(err);
})