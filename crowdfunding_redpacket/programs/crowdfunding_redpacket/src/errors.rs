// in errors.rs

use anchor_lang::prelude::*;

#[error_code]
pub enum RedPacketError {
    // --- 0. 通用错误 (General Errors) ---
    /// 算术运算溢出
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    /// 提供的创建者与记录不符
    #[msg("Invalid creator specified")]
    InvalidCreator,
    /// 金库余额不足
    #[msg("Insufficient vault balance")]
    InsufficientVaultBalance,

    // --- 1. 创建红包 (Create RedPacket) ---
    /// 发行的代币总量必须大于零
    #[msg("Total amount must be greater than zero")]
    InvalidTotalAmount,
    /// 代币名称不符合规范 (不能为空，长度不超32)
    #[msg("Invalid token name format")]
    InvalidTokenName,
    /// 代币符号不符合规范 (不能为空，长度不超10)
    #[msg("Invalid token symbol format")]
    InvalidTokenSymbol,
    /// 众筹目标必须大于零
    #[msg("Funding goal must be greater than zero")]
    InvalidFundingGoal,
    /// 众筹结束时间无效 (必须在未来)
    #[msg("Invalid expiry time")]
    InvalidExpiryTime,
    /// 自定义分配项过多 (超过最大限制)
    #[msg("Too many allocation entries")]
    TooManyAllocationTypes,
    /// 自定义分配项的金额必须大于零
    #[msg("Allocation amount must be greater than zero")]
    InvalidAllocationAmount,
    /// 自定义分配项的名称不符合规范
    #[msg("Invalid allocation name format")]
    InvalidAllocationName,
    /// 自定义分配项的总额与代币发行总量不符
    #[msg("Sum of custom allocations does not match total amount")]
    InvalidAllocation,
    /// 自定义分配项中存在重复的名称
    #[msg("Duplicate allocation name found")]
    DuplicateAllocationName,
    /// 自定义分配方案中缺少必须的“空投”部分
    #[msg("Missing 'airdrop' allocation in custom setup")]
    MissingAirdropAllocation,
    /// 自定义分配方案中缺少必须的“流动性”部分
    #[msg("Missing 'liquidity' allocation in custom setup")]
    MissingLiquidityAllocation,
    /// 空投最大人数必须大于零
    #[msg("Airdrop max count must be greater than zero")]
    InvalidAirdropMaxCount,
    /// 空投池的代币数量不足以满足最大可领取人数
    #[msg("Airdrop amount is too low for the max count of claimants")]
    AirdropAmountTooLowForMaxCount,

    // --- 2. 支持众筹 (Support Crowdfunding) ---
    /// 众筹活动已结束
    #[msg("Crowdfunding period has ended")]
    CrowdfundingEnded,
    /// 众筹项目已结算，无法再支持
    #[msg("Crowdfunding has been settled, cannot support anymore")]
    RedPacketSettled,
    /// 支持的金额无效
    #[msg("Invalid support amount")]
    InvalidSupportAmount,
    /// 该用户已经支持过此项目
    #[msg("This wallet has already supported the project")]
    AlreadySupported,
    /// 用户无贡献，无法执行此操作
    #[msg("No contribution found for this user")]
    NoContribution,

    // --- 3. 结算众筹 (Settle Crowdfunding) ---
    /// 众筹活动已结算，无法重复操作
    #[msg("Crowdfunding has already been settled")]
    AlreadySettled,
    /// 众筹活动尚未结束，无法结算
    #[msg("Crowdfunding has not ended yet")]
    CrowdfundingNotEnded,
    /// 众筹未募集到任何资金
    #[msg("No funds were raised")]
    NoFundsRaised,
    /// 众筹未成功，无法执行此操作
    #[msg("Crowdfunding was not successful")]
    CrowdfundingFailed,
    /// 结算时，未找到必须的“众筹奖励”分配条目
    #[msg("Missing 'crowdfunding' allocation for settlement")]
    MissingCrowdfundingAllocation,

    // --- 4. 领取与退款 (Claims & Refunds) ---
    /// 用户已经领取过空投
    #[msg("Airdrop has already been claimed by this user")]
    AlreadyClaimed,
    /// 空投名额已全部被领取
    #[msg("Airdrop supply has been exhausted")]
    AirdropExhausted,
    /// 项目未结算，无法开始领取
    #[msg("Crowdfunding not settled yet, cannot claim")]
    CrowdfundingNotSettled,
    /// 解锁期尚未开始
    #[msg("Unlock period has not started")]
    UnlockNotStarted,
    /// 此刻没有已解锁且未领取的众筹奖励代币
    #[msg("No vested crowdfunding tokens available to claim at this time")]
    NoVestedTokensToClaim,
    /// 用户已经退款
    #[msg("Already refunded for this project")]
    AlreadyRefunded,
    /// 退款窗口已关闭
    #[msg("Refund window is closed")]
    RefundWindowClosed,
    /// 众筹成功，项目不可退款
    #[msg("Project was successful, not refundable")]
    NotRefundable,

    // --- 5. 开发资金与协议费用 (Dev Fund & Fees) ---
    /// 未分配任何开发资金（金额为0）
    #[msg("No development fund has been allocated (amount is zero)")]
    NoDevFundAllocated,
    /// 此刻没有可领取的开发资金(SOL)
    #[msg("No developer fund (SOL) available to claim at this time")]
    NoDevFundToClaim,
    /// 开发资金解锁尚未开始
    #[msg("Developer fund vesting has not started yet")]
    DevFundNotStarted,
    /// 费用已经分配完毕，无法重复操作
    #[msg("Fees have already been distributed")]
    FeesAlreadyDistributed,
    /// 没有可分配的费用
    #[msg("No fees to distribute")]
    NoFeesToDistribute,
    /// 配置的费用百分比无效
    #[msg("Invalid fee percentage configuration")]
    InvalidFeePercentage,
    /// 未找到开发者资金的分配条目
    #[msg("Missing 'developer' allocation entry")]
    MissingDeveloperAllocation,
    /// 解锁周期必须大于0
    #[msg("Unlock months must be greater than zero")]
    InvalidUnlockMonths,

    // --- 6. 账户与程序校验 (Accounts & Programs Validation) ---
    /// 无效的 Mint 账户
    #[msg("Invalid mint account provided")]
    InvalidMint,
    /// 无效的 Token Program
    #[msg("Invalid token program provided")]
    InvalidTokenProgram,
    /// 金库的 Mint 与记录不符
    #[msg("Vault mint mismatch")]
    InvalidVaultMint,
    /// 金库的 Owner 与记录不符
    #[msg("Vault owner mismatch")]
    InvalidVaultOwner,
    /// 领币者的 ATA Mint 与记录不符
    #[msg("Claimer ATA mint mismatch")]
    InvalidClaimerATAMint,
    /// 领币者的 ATA Owner 与记录不符
    #[msg("Claimer ATA owner mismatch")]
    InvalidClaimerATAOwner,
    /// 创建者的代币账户 Mint 与记录不符
    #[msg("Creator token account mint mismatch")]
    InvalidTokenAccountMint,
    /// 创建者的代币账户 Owner 与记录不符
    #[msg("Creator token account owner mismatch")]
    InvalidTokenAccountOwner,

    // --- 7. 开发与占位符 (Development & Placeholders) ---
    /// Raydium 集成功能尚未实现
    #[msg("Raydium integration not implemented")]
    RaydiumNotImplemented,
}
