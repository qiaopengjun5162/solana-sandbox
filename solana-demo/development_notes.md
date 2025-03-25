# Solana

```bash
➜ solana-test-validator -r

Ledger location: test-ledger
Log: test-ledger/validator.log
⠄ Initializing...                                                                                                        Waiting for fees to stabilize 1...
Identity: AvV5zuhWHPCNRhq8qyPrfA4yjLqiFPySFxyYt7NuHicE
Genesis Hash: EQ6H4aqK3RbAP2eYtvrMFHK3SrbKT9NHJ4MtT2hrVCKB
Version: 2.0.15
Shred Version: 290
Gossip Address: 127.0.0.1:1024
TPU Address: 127.0.0.1:1027
JSON RPC URL: http://127.0.0.1:8899
WebSocket PubSub URL: ws://127.0.0.1:8900
⠁ 00:03:00 | Processed Slot: 382 | Confirmed Slot: 382 | Finalized Slot: 351

➜ ts-node transfer.ts

✅ - 已建立与 http://127.0.0.1:8899 的连接
(node:4171) ExperimentalWarning: The Ed25519 Web Crypto API algorithm is an experimental feature and might change at any time
(Use `node --trace-warnings ...` to show where the warning was created)
✅ - 新的 user1 地址已创建：3ziUaeQ1zZwbQf1FYHGYrKQkifEWSG5K47WKPQFzXVpp
✅ - 从文件生成 user2 地址：Sq7vmiewuLGcQArLUhny1iXtXPqrnC1EWGt5SJ89aS5
✅ - user1 使用 RPC 方法空投 1 SOL
✅ - tx1: 36RRUnQQbPYrXjSTcpw5Aqf9KYW1aBEp8GH9akBAgqfypeyxtWSLx9Ss3e7tqQ83sgwpdhtfaUTXbHBspk58z6dP
(node:4171) [UNDICI-WS] Warning: WebSockets are experimental, expect them to change at any time.
✅ - user2 使用工厂函数空投 1 SOL
✅ - tx2: 2YFtea2ez4XuDdgKn6Zj9L6ogdt9eFwu4CfSb4Ekgd7kbmUQJEU6frkvVKvCBjBL1fZWM3yaxoKetQyTLZjzsCnj
✅ - 转账交易: 3Wqnj7Whghqyg9xVXEBkFbTJvCjBJ83eHQ91FPoJ69KzBmWSSppYnvuz7kC9n2uJotVg4eB1ARBbgUQt6SbiHfxe
```
