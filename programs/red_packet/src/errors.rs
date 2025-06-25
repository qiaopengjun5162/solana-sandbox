use anchor_lang::prelude::*;

#[error_code]
pub enum RedPacketError {
    /* 参数校验类错误 (4xx) */
    #[msg("Invalid red packet type (must be 0-2)")]
    InvalidRedPacketType,
    #[msg("Claim amount out of valid range")]
    InvalidClaimAmount,
    #[msg("Expiry days must be 1-30")]
    InvalidExpiryDays,
    #[msg("Expiry time must be in the future and within 30 days")]
    InvalidExpiryTime,
    #[msg("Packet count must be at least 1")]
    InvalidPacketCount,
    #[msg("Total amount must cover all packets")]
    InsufficientTotalAmount,
    #[msg("Share amount cannot be zero")]
    InvalidShareAmount,
    #[msg("Random seed is required for random amount red packet")]
    RandomSeedRequired,
    #[msg("Packet count exceeds maximum limit")]
    PacketCountTooLarge,
    #[msg("Invalid token program")]
    InvalidTokenProgram,

    /* 状态校验类错误 (5xx) */
    #[msg("Red packet has expired")]
    RedPacketExpired,
    #[msg("Red packet not expired yet")]
    RedPacketNotExpired,
    #[msg("Already claimed by this user")]
    AlreadyClaimed,
    #[msg("No packets remaining")]
    NoPacketsRemaining,
    #[msg("No funds available for refund")]
    NoFundsToRefund,
    #[msg("Cannot change expiry time more than 3 times")]
    TooManyExpiryChanges,

    /* 权限类错误 (6xx) */
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Confidential transfer tokens are not supported")]
    ConfidentialTransferDisabled,
    #[msg("Token must have disabled mint authority")]
    MintAuthorityShouldBeDisabled,
    #[msg("Invalid mint account data")]
    InvalidMintAccount,

    /* 加密验证类错误 (7xx) */
    #[msg("Invalid merkle root format")]
    InvalidMerkleRoot,
    #[msg("Merkle proof verification failed")]
    MerkleProofInvalid,
    #[msg("Merkle proof length exceeds maximum")]
    MerkleProofTooLong,
    #[msg("Randomness generation error")]
    RandomnessError,

    /* 账户类错误 (8xx) */
    #[msg("Invalid mint account")]
    InvalidMint,
    #[msg("Invalid associated token account")]
    InvalidATA,
    #[msg("Insufficient funds in red packet")]
    InsufficientFunds,
    #[msg("Insufficient funds in claimer account")]
    InsufficientClaimerFunds,
    #[msg("Counter overflow")]
    CounterOverflow,
    #[msg("Invalid pool ATA")]
    InvalidPoolAta,
    #[msg("Invalid red packet ID")]
    InvalidRedPacketId,
    #[msg("Transfer hook is not supported")]
    TransferHookNotSupported,
    #[msg("Permanent delegate is not supported")]
    PermanentDelegateNotSupported,
    #[msg("Confidential transfer is not supported")]
    ConfidentialTransferNotSupported,
    #[msg("Non-transferable token is not supported")]
    NonTransferableNotSupported,
    #[msg("Invalid extension data")]
    InvalidExtension,
    #[msg("Invalid transfer fee calculation")]
    InvalidTransferFee,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    #[msg("Invalid fee calculation")]
    FeeCalculationError,
}
