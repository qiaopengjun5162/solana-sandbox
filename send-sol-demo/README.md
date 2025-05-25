# Web3 Solana

## How to send SOL

这段代码是一个用 Rust 编写的 Solana 区块链转账示例，主要流程如下：

### 1. 导入依赖

- `solana_client::nonblocking::rpc_client::RpcClient`：异步 RPC 客户端，用于与 Solana 节点通信。
- `solana_sdk` 相关模块：用于密钥对、签名、系统指令、交易等。

### 2. 主函数入口

- 使用 `#[tokio::main]` 宏，表示主函数是异步的，基于 Tokio 异步运行时。

### 3. 创建 RPC 客户端

```rust
let client = RpcClient::new_with_commitment(
    String::from("http://127.0.0.1:8899"),
    CommitmentConfig::confirmed(),
);
```

- 连接本地的 Solana 节点（假设本地已启动 solana-test-validator）。
- 使用“confirmed”确认级别。

### 4. 生成密钥对

```rust
let from_keypair = Keypair::new();
let to_keypair = Keypair::new();
```

- 随机生成两个密钥对，分别作为转账的发送方和接收方。

### 5. 构造转账指令

```rust
let transfer_ix = transfer(
    &from_keypair.pubkey(),
    &to_keypair.pubkey(),
    LAMPORTS_PER_SOL,
);
```

- 构造一个系统转账指令，从发送方向接收方转 1 SOL（1 SOL = 1_000_000_000 lamports）。

### 6. 请求空投

```rust
let transaction_signature = client
    .request_airdrop(&from_keypair.pubkey(), 5 * LAMPORTS_PER_SOL)
    .await?;
```

- 向本地节点请求给发送方账户空投 5 SOL，便于后续转账。

### 7. 等待空投确认

```rust
loop {
    if client.confirm_transaction(&transaction_signature).await? {
        break;
    }
}
```

- 循环等待，直到空投交易被确认。

### 8. 构造并签名交易

```rust
let mut transaction = Transaction::new_with_payer(&[transfer_ix], Some(&from_keypair.pubkey()));
transaction.sign(&[&from_keypair], client.get_latest_blockhash().await?);
```

- 构造一个包含转账指令的交易，并由发送方签名。

### 9. 发送并确认交易

```rust
match client.send_and_confirm_transaction(&transaction).await {
    Ok(signature) => println!("Transaction Signature: {}", signature),
    Err(err) => eprintln!("Error sending transaction: {}", err),
}
```

- 发送交易到链上，并等待确认。
- 成功则打印交易签名，失败则打印错误信息。

### 10. 结束

```rust
Ok(())
```

- 程序正常结束。

---

**总结**：  
这段代码演示了如何用 Rust 通过 Solana RPC 客户端实现账户空投和转账的完整流程，适合本地测试和学习 Solana 开发。

```bash
SolanaSandbox/send-sol-demo on  main [?] is 📦 0.1.0 via 🦀 1.86.0 took 25.9s 
➜ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.64s

SolanaSandbox/send-sol-demo on  main [?] is 📦 0.1.0 via 🦀 1.86.0 
➜ cargo run  
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.75s
     Running `target/debug/send-sol-demo`
Transaction Signature: 5vipgKKTHhN6RpZi65RLFHN31id3EfWpifxcMWic5DjCcWWP8DHg5y5hb8w5sCTwV2xsQR9M6CGJSQQfJkjhqgJy

SolanaSandbox/send-sol-demo on  main [?] is 📦 0.1.0 via 🦀 1.86.0 took 2.6s 
➜ tree . -L 6 -I "target|test-ledger"
.
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs

2 directories, 3 files

SolanaSandbox/send-sol-demo on  main [?] is 📦 0.1.0 via 🦀 1.86.0 
➜ touch README.md
```
