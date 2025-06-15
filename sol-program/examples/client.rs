use anyhow::Result;
use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_native_token::LAMPORTS_PER_SOL;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};
use std::{env, path::Path, str::FromStr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let program_id = Pubkey::from_str("GGBjDqYdicSE6Qmtu6SAsueX1biM5LjbJ8R8vZvFfofA")?;

    // let rpc_url = String::from("http://127.0.0.1:8899");
    let rpc_url =
        env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let commitment_config = CommitmentConfig::confirmed();
    let rpc_client = RpcClient::new_with_commitment(rpc_url, commitment_config);

    // let keypair = Keypair::new();
    let keypair = load_keypair("./keys/SSoyAkBN9E3CjbWpr2SdgLa6Ejbqqdvasuxd8j1YsmN.json")?;
    println!("Keypair: {}", keypair.pubkey());

    let balance = rpc_client.get_balance(&keypair.pubkey())?;
    println!("Balance: {} SOL", balance as f64 / LAMPORTS_PER_SOL as f64);
    if balance < LAMPORTS_PER_SOL {
        println!("Requesting airdrop...");
        let signature = rpc_client
            .request_airdrop(&keypair.pubkey(), 2 * LAMPORTS_PER_SOL)
            .expect("Failed to request airdrop");
        loop {
            let confirmed =
                rpc_client.confirm_transaction_with_commitment(&signature, commitment_config)?;
            if confirmed.value {
                break;
            }
        }

        println!("Airdrop received");
    }

    let instruction = Instruction::new_with_borsh(program_id, &(), vec![]);

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&keypair.pubkey()));
    transaction.sign(&[&keypair], rpc_client.get_latest_blockhash()?);

    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction Signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }
    Ok(())
}

fn load_keypair(path: &str) -> anyhow::Result<Keypair> {
    if !Path::new(path).exists() {
        anyhow::bail!("Keypair file does not exist: {}", path);
    }
    read_keypair_file(path).map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))
}
