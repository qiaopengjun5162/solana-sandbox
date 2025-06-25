use anchor_lang::prelude::*;

#[event]
pub struct RedPacketCreated {
    pub creator: Pubkey,
    pub red_packet: Pubkey,
    pub total_amount: u64,
    pub packet_count: u32,
    pub red_packet_type: u8,
    pub expiry_time: i64,
    pub is_sol: bool,
    pub red_packet_id: u64,
    pub bump: u8,
    pub mint: Pubkey,
    pub has_transfer_fee: bool,
    pub has_transfer_hook: bool,
    pub has_permanent_delegate: bool,
    pub has_close_authority: bool,
}

#[event]
pub struct RedPacketClaimed {
    pub claimer: Pubkey,
    pub red_packet: Pubkey,
    pub amount: u64,
    pub red_packet_id: u64,
}

#[event]
pub struct RedPacketRefunded {
    pub creator: Pubkey,
    pub red_packet: Pubkey,
    pub amount: u64,
    pub red_packet_id: u64,
}

#[event]
pub struct ExpiryTimeUpdated {
    pub red_packet: Pubkey,
    pub new_expiry_time: i64,
    pub red_packet_id: u64,
}
