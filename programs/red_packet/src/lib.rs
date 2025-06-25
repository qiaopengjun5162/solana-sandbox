#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("7rSdaJc2nJafXjKD39nxmhkmCexUFQsCisg42oyRsqvt");

pub mod config;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod states;
pub mod utils;

use errors::*;
use instructions::*;
use states::*;

#[program]
pub mod red_packet {
    use super::*;

    pub fn initialize_creator_state(ctx: Context<InitializeCreatorState>) -> Result<()> {
        instructions::initialize::handler_creator_state(ctx)
    }

    pub fn create_redpacket(
        ctx: Context<CreateRedPacket>,
        total_amount: u64,
        packet_count: u32,
        red_packet_type: u8,
        merkle_root: Option<[u8; 32]>,
        is_sol: bool,
        expiry_days: Option<i64>,
        random_seed: Option<u64>,
    ) -> Result<()> {
        instructions::create::create_handler(
            ctx,
            total_amount,
            packet_count,
            red_packet_type,
            merkle_root,
            is_sol,
            expiry_days,
            random_seed,
        )
    }

    pub fn claim_redpacket(
        ctx: Context<ClaimRedPacket>,
        amount: Option<u64>,
        proof: Option<Vec<[u8; 32]>>,
        red_packet_id: u64,
    ) -> Result<()> {
        instructions::claim::claim_handler(ctx, amount, proof, red_packet_id)
    }

    pub fn refund(ctx: Context<Refund>, red_packet_id: u64) -> Result<()> {
        instructions::refund::refund_handler(ctx, red_packet_id)
    }

    pub fn set_expiry_time(ctx: Context<SetExpiryTime>, expiry_time: i64) -> Result<()> {
        instructions::expiry::expiry_handler(ctx, expiry_time)
    }
}
