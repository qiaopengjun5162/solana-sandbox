use anchor_lang::prelude::*;

use crate::state::AllocationEntry;

#[event]
pub struct AirdropClaimed {
    pub claimer: Pubkey,
    pub red_packet: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct CrowdfundingSupported {
    pub backer: Pubkey,
    pub red_packet: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct CrowdfundingSettled {
    pub red_packet: Pubkey,
    pub success: bool,
    pub sol_raised: u64,
    pub liquidity_sol_amount: u64,
    pub liquidity_token_amount: u64,
    pub dev_fund_sol_amount: u64,
    pub creator_direct_amount: u64,
    pub protocol_fee_amount: u64,
    pub liquidity_pool: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct Refunded {
    pub backer: Pubkey,
    pub red_packet: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct TokensClaimed {
    pub backer: Pubkey,
    pub red_packet: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct DevFundClaimed {
    pub creator: Pubkey,
    pub red_packet: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct FeesDistributed {
    // 哪个红包项目
    pub red_packet: Pubkey,
    // 由谁触发的分配（也就是创建者）
    pub trigger: Pubkey,
    // 分配的总金额
    pub total_distributed: u64,
    // 创建者分到的金额
    pub creator_fee: u64,
    // 开发者分到的金额
    pub developer_fee: u64,
    // 接收开发者费用的钱包地址
    pub developer_wallet: Pubkey,
    // 时间戳
    pub timestamp: i64,
}

#[event]
pub struct RedPacketCreated {
    pub creator: Pubkey,
    pub red_packet: Pubkey,

    // --- 关键配置参数 ---
    pub name: String,
    pub symbol: String,
    pub funding_goal: u64,
    pub expiry_time: i64,

    // --- 代币分配信息 ---
    pub total_supply: u64,
    pub allocations: Vec<AllocationEntry>,

    // --- 时间戳 ---
    pub timestamp: i64, // 创建事件也应该有时间戳
}
