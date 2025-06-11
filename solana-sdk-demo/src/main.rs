use std::env;
use std::path::Path;

use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::system_instruction::create_account;
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;
use spl_token::ID as TOKEN_PROGRAM_ID;
use spl_token::instruction::initialize_mint2;
use spl_token_2022::extension::BaseStateWithExtensions;
use spl_token_2022::extension::StateWithExtensions;
use spl_token_2022::state::Account as TokenAccount;
use spl_token_2022::state::Mint;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 加载环境变量
    dotenv().ok();

    // 2. 初始化RPC客户端
    let rpc_url = env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".into());
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // 3. 加载支付账户
    let authority_key_path = "./keys/SdkcwrahL9q1wm7qB8wQwNBKxZJ4Ck7A413tWDkSBjo.json";
    let authority_keypair = load_keypair(authority_key_path)?;

    // 4. 检查并充值测试SOL
    let balance = client.get_balance(&authority_keypair.pubkey())?;
    println!("当前余额: {} SOL", balance as f64 / LAMPORTS_PER_SOL as f64);

    if balance < 1_000_000_000 {
        // 如果余额小于1 SOL
        airdrop(&client, &authority_keypair.pubkey(), 1)?; // 空投1 SOL
    }

    // 4. 生成全新的密钥对（专门用于Mint）
    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();

    // 6. 构建交易指令
    let mint_account_len = Mint::LEN;
    let rent = client.get_minimum_balance_for_rent_exemption(mint_account_len)?;

    let instructions = vec![
        // 创建账户指令
        create_account(
            &authority_keypair.pubkey(),
            &mint_pubkey,
            rent,
            mint_account_len as u64,
            &TOKEN_PROGRAM_ID,
        ),
        // 初始化Mint指令
        initialize_mint2(
            &TOKEN_PROGRAM_ID,
            &mint_pubkey,
            &authority_keypair.pubkey(),       // Mint权限
            Some(&authority_keypair.pubkey()), // 冻结权限（设为同一地址）
            9,                                 // 代币小数位
        )?,
        // 创建Token账户 create_ata_ix
        create_associated_token_account_idempotent(
            &authority_keypair.pubkey(), // payer
            &authority_keypair.pubkey(),
            &mint_pubkey,
            &TOKEN_PROGRAM_ID,
        ),
    ];

    // 7. 发送交易
    let mut tx = Transaction::new_with_payer(&instructions, Some(&authority_keypair.pubkey()));

    tx.sign(
        &[&authority_keypair, &mint_keypair],
        client.get_latest_blockhash()?,
    );

    match client.send_and_confirm_transaction(&tx) {
        Ok(signature) => println!("Transaction Signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }

    let mint_data = client.get_account_data(&mint_pubkey)?;
    let mint = StateWithExtensions::<Mint>::unpack(&mint_data).unwrap();
    let extension_types = mint.get_extension_types().unwrap();

    println!("Mint pubkey: {}", mint_pubkey);
    println!("Mint: {:#?}", mint);
    println!("Extension types: {:#?}", extension_types);

    let token_account_address = get_associated_token_address(
        &authority_keypair.pubkey(), // 当前用户
        &mint_pubkey,                // 刚创建的Mint
    );
    let token_account_data = client.get_token_account(&token_account_address)?;
    println!("token_account_data: {token_account_data:#?}");

    let account_data = client.get_account_data(&token_account_address)?;
    let token_account_data = TokenAccount::unpack(&account_data)?;
    println!("Token Account Data: {token_account_data:#?}");

    let balance = client.get_token_account_balance(&token_account_address)?;
    println!("Token Account Balance: {:#?}", balance);

    Ok(())
}

// 辅助函数：加载密钥对
fn load_keypair(path: &str) -> anyhow::Result<Keypair> {
    if !Path::new(path).exists() {
        anyhow::bail!("密钥文件不存在: {}", path);
    }
    read_keypair_file(path).map_err(|e| anyhow::anyhow!("读取密钥失败: {}", e))
}

// 辅助函数：请求空投
fn airdrop(client: &RpcClient, address: &Pubkey, sol: u64) -> anyhow::Result<()> {
    let signature = client.request_airdrop(address, sol * LAMPORTS_PER_SOL)?;

    // 等待确认
    loop {
        if client.confirm_transaction(&signature)? {
            println!("🪂 成功空投 {} SOL", sol);
            break;
        }
    }
    Ok(())
}
