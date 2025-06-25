use anchor_lang::prelude::*;

#[account]
pub struct RedPacket {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub total_amount: u64,
    pub remaining_amount: u64,
    pub packet_count: u32,
    pub claimed_count: u32,
    pub red_packet_type: u8,
    pub share_amount: u64,
    pub random_seed: u64,
    pub expiry_time: i64,
    pub merkle_root: [u8; 32],
    pub is_sol: bool,
    pub expiry_time_changes: u8,
    pub red_packet_id: u64,
    pub bump: u8,
}

#[account]
pub struct UserState {
    pub is_claimed: u8, // 1 表示已领取，0 表示未领取
}

#[account]
pub struct CreatorState {
    pub next_red_packet_id: u64,
    pub bump: u8,
}
