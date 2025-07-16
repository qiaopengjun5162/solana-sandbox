use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self as spl_token},
    token_2022,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{constants::config::MAX_ALLOCATION_COUNT, errors::RedPacketError};

// 空投状态
#[account]
pub struct AirdropState {
    pub claimed: bool, // 是否已领取
    pub bump: u8,      // bump seed
}

// 支持者状态
#[account]
pub struct BackerState {
    pub amount: u64,                     // 支持的 SOL 数量 (8 字节)
    pub refunded: bool,                  // 是否已退款 (1 字节)
    pub claimed_amount: u64,             // 已领取的奖励代币数量 (8 字节)
    pub unlock_scheme: UnlockSchemeType, // 解锁方案类型 (1 字节 + padding)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum UnlockSchemeType {
    Immediate, // 对应 0.05 SOL，立即解锁
    Gradual,   // 对应 0.5 SOL，渐进解锁
}

#[account]
#[derive(Default)]
pub struct SolVault {}

#[derive(Accounts)]
#[instruction(params: CustomCrowdfundingParams)]
pub struct CreateCustomRedpacket<'info> {
    /// CHECK: This is the creator and must be a signer.
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = 8 + std::mem::size_of::<RedPacket>() + MAX_ALLOCATION_COUNT * std::mem::size_of::<AllocationEntry>(),
        seeds = [b"red_packet", creator.key().as_ref()],
        bump
    )]
    pub red_packet: Account<'info, RedPacket>,
    #[account(
        mut,
        constraint = creator_token_account.mint == mint.key() @ RedPacketError::InvalidTokenAccountMint,
        constraint = creator_token_account.owner == creator.key() @ RedPacketError::InvalidTokenAccountOwner,
    )]
    pub creator_token_account: InterfaceAccount<'info, TokenAccount>,

    // system_program::transfer 这个工具，不适用于“合约程序从自己控制的 PDA 金库里转出 SOL”这个特定场景。
    /// CHECK: This is a PDA controlled by the program, verified by seeds and bump.
    #[account(
        init,
        payer = creator,
        space = 8,
        seeds = [b"sol_vault", red_packet.key().as_ref()],
        bump
    )]
    pub sol_vault: Account<'info, SolVault>, // ：PDA 账户（AccountInfo），用于存储众筹的 SOL（通过 support_crowdfunding），在 settle_crowdfunding 或 refund 中分配或退款。
    #[account(
        init,
        payer = creator,
        token::mint = mint,
        token::authority = red_packet,
        seeds = [b"token_vault", red_packet.key().as_ref()],
        bump
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>, // 代币账户（InterfaceAccount<TokenAccount>），用于存储创建者的代币（total_amount），供空投（claim_airdrop）、众筹奖励（claim_tokens）和流动性池（settle_crowdfunding）使用。
    #[account(
        constraint = mint.key() == params.mint @ RedPacketError::InvalidMint,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    #[account(
        constraint = token_program.key() == spl_token::ID || token_program.key() == token_2022::ID
            @ RedPacketError::InvalidTokenProgram
    )]
    pub token_program: Interface<'info, TokenInterface>,
}

// 领取空投上下文
#[derive(Accounts)]
#[instruction()]
pub struct ClaimAirdrop<'info> {
    #[account(
        mut,
        seeds = [b"red_packet", creator.key().as_ref()],
        bump,
    )]
    pub red_packet: Account<'info, RedPacket>,
    #[account(mut)]
    pub claimer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = claimer,
        space = 8 + std::mem::size_of::<AirdropState>(),
        seeds = [b"airdrop", red_packet.key().as_ref(), claimer.key().as_ref()],
        bump
    )]
    pub airdrop_state: Account<'info, AirdropState>,
    #[account(
        mut,
        seeds = [b"token_vault", red_packet.key().as_ref()],
        bump,
        constraint = token_vault.mint == red_packet.mint @ RedPacketError::InvalidVaultMint,
        constraint = token_vault.owner == red_packet.key() @ RedPacketError::InvalidVaultOwner,
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = claimer_ata.mint == red_packet.mint @ RedPacketError::InvalidClaimerATAMint,
        constraint = claimer_ata.owner == claimer.key() @ RedPacketError::InvalidClaimerATAOwner,
    )]
    pub claimer_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        constraint = mint.key() == red_packet.mint @ RedPacketError::InvalidMint,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK: Verified by constraint that creator matches red_packet.creator.
    #[account(
        constraint = creator.key() == red_packet.creator @ RedPacketError::InvalidCreator,
    )]
    pub creator: AccountInfo<'info>,
    #[account(
        constraint = token_program.key() == spl_token::ID || token_program.key() == token_2022::ID
            @ RedPacketError::InvalidTokenProgram
    )]
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct SupportCrowdfunding<'info> {
    #[account(
        mut,
        seeds = [b"red_packet", creator.key().as_ref()],
        bump,
    )]
    pub red_packet: Account<'info, RedPacket>,
    #[account(mut)]
    pub backer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = backer,
        space = 8 + 8 + 2 + 8 + 1, // 8(disc) + 8(amount) + 2(unlock_scheme) + 8(claimed) + 1(refunded)
        seeds = [b"backer_state", red_packet.key().as_ref(), backer.key().as_ref()],
        bump
    )]
    pub backer_state: Account<'info, BackerState>,
    /// CHECK: This is a PDA controlled by the program, verified by seeds and bump.
    #[account(
        mut,
        seeds = [b"sol_vault", red_packet.key().as_ref()],
        bump,
    )]
    pub sol_vault: Account<'info, SolVault>,
    /// CHECK: 验证创建者与 red_packet.creator 一致
    /// CHECK: 通过约束条件验证 creator 必须等于 red_packet.creator，确保调用者权限正确。
    #[account(
        constraint = creator.key() == red_packet.creator @ RedPacketError::InvalidCreator,
    )]
    pub creator: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct SettleCrowdfunding<'info> {
    #[account(
        mut,
        seeds = [b"red_packet", creator.key().as_ref()],
        bump,
    )]
    pub red_packet: Account<'info, RedPacket>,
    /// CHECK: 必须是 red_packet 的创建者才能进行结算
    #[account(
        mut,
        constraint = creator.key() == red_packet.creator @ RedPacketError::InvalidCreator,
    )]
    pub creator: Signer<'info>,
    /// CHECK: This is a PDA controlled by the program, verified by seeds and bump.
    /// 存储众筹的 SOL，用于结算或分配给创建者。
    #[account(
        mut,
        seeds = [b"sol_vault", red_packet.key().as_ref()],
        bump,
    )]
    pub sol_vault: Account<'info, SolVault>,
    #[account(
        mut,
        seeds = [b"token_vault", red_packet.key().as_ref()],
        bump,
        constraint = token_vault.mint == red_packet.mint @ RedPacketError::InvalidVaultMint,
        constraint = token_vault.owner == red_packet.key() @ RedPacketError::InvalidVaultOwner,
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    #[account(
        constraint = token_program.key() == spl_token::ID || token_program.key() == token_2022::ID
            @ RedPacketError::InvalidTokenProgram
    )]
    pub token_program: Interface<'info, TokenInterface>,
    // TODO: 添加 Raydium 相关账户
    // pub raydium_program: Program<'info, Raydium>,
    // pub amm_pool: AccountInfo<'info>,
    // pub token_vault: InterfaceAccount<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(
        mut,
        // 明确地使用 creator.key() 来验证 seeds
        seeds = [b"red_packet", creator.key().as_ref()],
        bump,
    )]
    pub red_packet: Account<'info, RedPacket>,
    #[account(mut)]
    pub backer: Signer<'info>,
    #[account(mut)]
    pub backer_state: Account<'info, BackerState>,
    /// CHECK: 这是一个由程序控制的 PDA 账户，通过 seeds 和 bump 验证安全性。
    /// 存储众筹的 SOL，用于退款给支持者。
    #[account(
        mut,
        seeds = [b"sol_vault", red_packet.key().as_ref()],
        bump,
    )]
    pub sol_vault: Account<'info, SolVault>,
    /// CHECK: 传入 creator 以便 Anchor 验证 red_packet PDA
    #[account(constraint = creator.key() == red_packet.creator @ RedPacketError::InvalidCreator)]
    pub creator: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(
        mut,
        seeds = [b"red_packet", creator.key().as_ref()],
        bump,
    )]
    pub red_packet: Account<'info, RedPacket>,
    #[account(mut)]
    pub claimer: Signer<'info>,
    #[account(mut)]
    pub backer_state: Account<'info, BackerState>,
    #[account(
        mut,
        seeds = [b"token_vault", red_packet.key().as_ref()],
        bump,
        constraint = token_vault.mint == red_packet.mint @ RedPacketError::InvalidVaultMint,
        constraint = token_vault.owner == red_packet.key() @ RedPacketError::InvalidVaultOwner,
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = claimer_ata.mint == red_packet.mint @ RedPacketError::InvalidClaimerATAMint,
        constraint = claimer_ata.owner == claimer.key() @ RedPacketError::InvalidClaimerATAOwner,
    )]
    pub claimer_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        constraint = mint.key() == red_packet.mint @ RedPacketError::InvalidMint,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK: 通过约束条件验证 creator 必须是 red_packet 的创建者
    #[account(
        constraint = creator.key() == red_packet.creator @ RedPacketError::InvalidCreator,
    )]
    pub creator: AccountInfo<'info>,
    #[account(
        constraint = token_program.key() == spl_token::ID || token_program.key() == token_2022::ID
            @ RedPacketError::InvalidTokenProgram
    )]
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct ClaimDevFund<'info> {
    #[account(mut)]
    pub red_packet: Account<'info, RedPacket>,
    /// CHECK: 必须是 red_packet 的创建者才能提取开发资金
    #[account(mut)]
    pub creator: Signer<'info>, // 核心原则：谁发起，谁签名；谁被引用，谁就是信息
    /// CHECK: 开发资金池账户，由程序控制的 PDA，安全性通过种子验证
    #[account(
        mut,
        seeds = [b"sol_vault", red_packet.key().as_ref()],
        bump,
    )]
    pub sol_vault: Account<'info, SolVault>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeFees<'info> {
    #[account(
        mut,
        seeds = [b"red_packet", creator.key().as_ref()], // 同样需要 creator 来验证
        bump
    )]
    pub red_packet: Account<'info, RedPacket>,
    /// CHECK: 创建者，用于验证 PDA，也是收款方之一
    #[account(
        mut, // creator 作为收款方，需要 mut
        constraint = creator.key() == red_packet.creator @ RedPacketError::InvalidCreator
    )]
    pub creator: Signer<'info>,

    #[account(seeds = [b"config"], bump)]
    pub config: Account<'info, Config>,

    // 强制要求这个账户的地址必须等于 config.developer_wallet 中记录的地址
    #[account(mut, address = config.developer_wallet)]
    pub developer_wallet: SystemAccount<'info>,

    /// CHECK: 手续费分配池账户，由程序控制的 PDA，安全性通过种子验证
    #[account(
        mut,
        seeds = [b"sol_vault", red_packet.key().as_ref()],
        bump,
    )]
    pub sol_vault: Account<'info, SolVault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize {}

// 红包账户 RedPacket 结构体
// 这是一个账户状态结构体（Account State Struct），用 #[account] 宏标记。它定义了每一个众筹红包项目在链上存储的所有数据。
#[account]
pub struct RedPacket {
    // === 身份与基础信息 ===
    pub creator: Pubkey,      // 创建者钱包地址 (32)
    pub mint: Pubkey,         // 项目代币的Mint地址 (32)
    pub token_name: String,   // 代币名称 (4 + 32)
    pub token_symbol: String, // 代币符号 (4 + 10)

    // === 项目代币经济学 (Tokenomics) ===
    pub total_amount: u64,                 // 本次活动发行的代币总量 (8)
    pub allocations: Vec<AllocationEntry>, // 代币的详细分配方案 (4 + N * size) - 这是项目代币分配的唯一数据源

    // === 众筹核心参数 ===
    pub funding_goal: u64,    // 众筹目标 (SOL lamports) (8)
    pub sol_raised: u64,      // 当前已筹集到的 SOL (lamports) (8)
    pub expiry_time: i64,     // 活动结束的Unix时间戳 (8)
    pub tokens_per_sol: u128, // SOL 与项目代币的兑换率 (16)

    // === 状态与时间戳 ===
    pub settled: bool,            // 标记活动是否已结算 (1)
    pub success: bool,            // 标记众筹是否成功 (1)
    pub fees_distributed: bool,   // 标记协议费用是否已分配 (1)
    pub unlock_start_time: i64,   // 全局代币解锁开始时间戳 (8)
    pub dev_fund_start_time: i64, // 开发资金(SOL)解锁开始时间戳 (8)

    // === 空投特定状态 ===
    pub airdrop_max_count: u16, // 允许领取空投的最大人数 (2)
    pub airdrop_claimed: u16,   // 当前已经领取空投的人数 (2)

    // === 结算后【SOL】的分配结果 (在 settle 指令中填充) ===
    pub creator_direct_amount: u64, // 直接分配给创建者的 SOL (8)
    pub liquidity_sol_amount: u64,  // 用于注入流动性的 SOL (8)
    pub protocol_fee_amount: u64,   // 平台收取的手续费 SOL (8)
    pub dev_fund_sol_amount: u64,   // 分配给开发资金的 SOL

    // === 结算后【项目代币】的分配结果 (在 settle 指令中填充) ===
    pub liquidity_token_amount: u64, // 用于注入流动性的项目代币数量 (8)

    // === 领取状态追踪 ===
    pub dev_fund_claimed: u64, // 创建者已领取的开发资金 SOL (8)

    // === 杂项配置与状态 ===
    pub liquidity_pool: Pubkey, // 创建的 Raydium 流动性池地址 (32)
    pub liquidity_fee_creator_percent: u64, // 流动性费用分成比例 (8)
}

// AllocationEntry 结构体
// 这是一个辅助数据结构，它没有 #[account] 宏，意味着它本身不会成为一个独立的Solana账户。
// 它被用在 RedPacket 结构体的 allocations 向量（Vec）中，用来定义每一份资金的用途。
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AllocationEntry {
    pub name: String,      // 分配项的名称，例如 "空投", "团队", "流动性" (动态大小)
    pub amount: u64,       // 分配给该项的代币数量 (8字节)
    pub unlock_months: u8, // 该部分代币的锁仓月数 (1字节)
}

// CustomCrowdfundingParams 结构体
// 这是一个指令参数结构体（Instruction Parameters Struct）。
// 它定义了当用户调用 create_custom_redpacket 指令时，需要从客户端（例如前端网页）传入的数据。
// 这些数据不会被直接存储在链上，而是被指令逻辑用来初始化 RedPacket 账户。
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CustomCrowdfundingParams {
    // === 核心参数，用于初始化 RedPacket 账户 ===
    pub mint: Pubkey,         // 要发行的代币地址
    pub total_amount: u64,    // 代币发行总量
    pub token_name: String,   // 代币名称
    pub token_symbol: String, // 代币符号
    pub funding_goal: u64,    // 众筹目标 (SOL)

    // === 可选/自定义参数 ===
    pub allocations: Vec<AllocationEntry>, // 用户自定义的代币分配方案
    pub airdrop_max_count: Option<u16>,    // （可选）空投最大数量，如果不提供则使用默认值
    pub expiry_duration: Option<i64>,      // （可选）众筹持续时长（秒），如果不提供则使用默认值
}

/// 全局配置账户，用于存储可由管理员更新的参数
#[account]
#[derive(Default)]
pub struct Config {
    /// 拥有更新配置权限的管理员地址
    pub admin: Pubkey,
    /// 接收平台费用的开发者/平台方钱包地址
    pub developer_wallet: Pubkey,
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    // 签名者即为初始的管理员
    #[account(mut)]
    pub admin: Signer<'info>,

    // 初始化 Config 账户
    // 它是一个 PDA，种子是固定的 "config"，确保全局只有一个
    #[account(
        init,
        payer = admin,
        space = 8 + std::mem::size_of::<Config>(),
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    // 必须是当前记录在 config 账户中的 admin 签名
    #[account(mut)]
    pub admin: Signer<'info>,

    // 加载要修改的 Config 账户
    // constraint 确保了只有合法的 admin 才能修改它
    #[account(
        mut,
        seeds = [b"config"],
        bump,
        constraint = config.admin == admin.key()
    )]
    pub config: Account<'info, Config>,
}
