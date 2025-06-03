import { createClient } from "./client";

const main = async () => {
  const client = await createClient();
  const { value: balance } = await client.rpc
    .getBalance(client.wallet.address)
    .send();
  console.log(`Balance: ${balance} lamports.`);
};

main().catch(console.error);
