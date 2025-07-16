# Crowdfunding redpacket  dapp

这是一个基于 Anchor 框架的 Solana 众筹红包程序，主要功能包括自定义代币分配、空投、众筹支持、流动性提供等。

## 工作流程

1. 项目方：创建红包并设置参数
2. 用户：

- 领取空投（如果有）
- 选择支持金额参与众筹

3. 众筹结束：

- 成功：代币按计划解锁，创建流动性池
- 失败：用户可申请退款

4. 代币解锁：

- 支持者按时间领取代币
- 开发者按月领取开发基金

## 文件结构

```bash
crowdfunding_redpacket/
├── lib.rs                # 入口文件，定义程序并重导出模块
├── instructions/         # 指令处理逻辑
│   ├── mod.rs            # 导出指令模块
│   ├── initialize.rs     # initialize 指令
│   ├── create.rs         # create_custom_redpacket 指令
│   ├── airdrop.rs        # claim_airdrop 指令
│   ├── support.rs        # support_crowdfunding 指令
│   ├── settle.rs         # settle_crowdfunding 指令
│   ├── refund.rs         # refund 指令
│   ├── claim_tokens.rs   # claim_tokens 指令
│   ├── claim_dev_fund.rs # claim_dev_fund 指令
│   └── distribute_fees.rs# distribute_fees 指令
├── accounts/             # 账户结构和上下文
│   ├── mod.rs            # 导出账户模块
│   ├── red_packet.rs     # RedPacket 和 AllocationEntry 结构
│   ├── backer_state.rs   # BackerState 结构
│   ├── airdrop_state.rs  # AirdropState 结构
│   └── contexts.rs       # 指令上下文（#[derive(Accounts)]）
├── errors.rs             # 错误码（RedPacketError 枚举）
├── constants.rs          # 常量（MAX_ALLOCATION_COUNT 等）
├── events.rs             # 事件定义（AirdropClaimed 等）
└── utils.rs              # 辅助函数（calculate_claimable_amount 等）
```
