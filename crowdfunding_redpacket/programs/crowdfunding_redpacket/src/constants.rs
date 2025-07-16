/// 全局配置
pub mod config {
    /// 一个红包项目中允许的最大分配类型数量
    pub const MAX_ALLOCATION_COUNT: usize = 10;
    /// 认领空投时需要支付的费用（例如用于创建状态账户）
    pub const CLAIM_FEE_LAMPORTS: u64 = 1_000_000; // 0.001 SOL
}

/// 时间相关常量
pub mod time {
    /// 一天中的秒数
    pub const SECONDS_IN_A_DAY: i64 = 86_400; // 24 * 60 * 60
    /// 众筹失败后的退款窗口期
    pub const REFUND_WINDOW_SECS: i64 = 30 * SECONDS_IN_A_DAY; // 30 天
    /// 用于计算线性解锁的“月”，近似为30天
    pub const SECONDS_PER_MONTH: i64 = 30 * SECONDS_IN_A_DAY; // 30 天
}

/// 空投相关默认配置
pub mod airdrop {
    /// 默认的空投最大可领取人数
    pub const DEFAULT_MAX_COUNT: u16 = 100;
}

/// 分配方案相关常量
pub mod allocations {
    /// 用于标识“空投”分配的名称
    pub const AIRDROP_NAME: &str = "airdrop";
    /// 用于标识“流动性”分配的名称
    pub const LIQUIDITY_NAME: &str = "liquidity";
    /// 用于标识“众筹奖励”分配的名称
    pub const CROWDFUNDING_NAME: &str = "crowdfunding";
    /// 用于标识“开发者”分配的名称
    pub const DEVELOPER_NAME: &str = "developer";

    /// 默认的项目代币分配百分比
    /// 注意：所有字段加起来应等于 100
    pub struct DefaultTokenPercentages {
        pub airdrop: u64,
        pub crowdfunding: u64,
        pub liquidity: u64,
        pub developer: u64,
    }

    pub const DEFAULT_TOKEN_PERCENTAGES: DefaultTokenPercentages = DefaultTokenPercentages {
        airdrop: 10,
        crowdfunding: 40,
        liquidity: 30,
        developer: 20,
    };

    /// 众筹成功后，募集到的 SOL 的分配百分比
    /// 注意：所有字段加起来应等于 100
    pub struct SettledSolPercentages {
        pub liquidity: u64,
        pub dev_fund: u64,
        pub creator: u64,
        pub protocol: u64,
    }

    pub const SETTLED_SOL_PERCENTAGES: SettledSolPercentages = SettledSolPercentages {
        liquidity: 70,
        dev_fund: 15,
        creator: 10,
        protocol: 5,
    };
}

/// 用户贡献与代币解锁方案
pub mod vesting {
    /// 为小额支持者设计的解锁方案
    /// 格式为：(距离解锁开始的天数, 解锁的百分点)
    pub const SMALL_SUPPORT_UNLOCK_SCHEME: &[(u32, u8)] = &[
        (0, 100), // 立即解锁 100%
    ];

    /// 为大额支持者设计的解锁方案
    /// 注意：所有百分点加起来应等于 100
    pub const LARGE_SUPPORT_UNLOCK_SCHEME: &[(u32, u8)] = &[
        (0, 40),   // 立即解锁 40%
        (30, 12),  // 30 天后解锁 12%
        (60, 12),  // 60 天后解锁 12%
        (90, 12),  // 90 天后解锁 12%
        (120, 12), // 120 天后解锁 12%
        (150, 12), // 150 天后解锁 12%
    ];
}

/// 支持众筹的层级与金额定义
pub mod support_tiers {
    /// 小额支持的金额 (0.05 SOL)
    pub const SMALL_SUPPORT_AMOUNT: u64 = 50_000_000;
    /// 大额支持的金额 (0.5 SOL)
    pub const LARGE_SUPPORT_AMOUNT: u64 = 500_000_000;
    /// 当众筹目标剩余金额小于此阈值时，只允许小额支持
    pub const MIN_SUPPORT_THRESHOLD: u64 = 500_000_000; // 0.5 SOL
}
