use anchor_lang::prelude::*;

pub const CLAIM_FEE: u64 = 1_000_000; // 0.001 SOL
pub const CREATE_FEE: u64 = 5_000_000; // 0.005 SOL
pub const MAX_PROOF_LENGTH: usize = 32;
pub const DEFAULT_RED_PACKET_EXPIRY_DAYS: i64 = 7;
pub const MAX_EXPIRY_TIME_CHANGES: u8 = 3;
pub const FEE_RECEIVER_SEED: &[u8] = b"fee_receiver";
pub const CREATOR_STATE_SEED: &[u8] = b"creator_state";
pub const RED_PACKET_SPACE: usize =
    8 + 32 + 32 + 8 + 8 + 4 + 4 + 1 + 8 + 8 + 8 + 32 + 1 + 1 + 8 + 1;
pub const USER_STATE_SPACE: usize = 8 + 1;
pub const FEE_VAULT_SPACE: usize = 8 + 8;
pub const CREATOR_STATE_SPACE: usize = 8 + 8 + 1;
pub const FEE_RECEIVER: Pubkey = pubkey!("15hPXzWgid1UWUKnp4KvtZEbaNUCWkPK79cb5uqHysf");
pub const MAX_PACKET_COUNT: u32 = 100000;
