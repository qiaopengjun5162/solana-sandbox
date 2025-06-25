#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

use crate::{config, events::ExpiryTimeUpdated, RedPacket, RedPacketError};

pub fn expiry_handler(ctx: Context<SetExpiryTime>, expiry_time: i64) -> Result<()> {
    let red_packet = &mut ctx.accounts.red_packet;
    let red_packet_key = red_packet.key();
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        ctx.accounts.authority.key() == red_packet.creator,
        RedPacketError::Unauthorized
    );
    require!(
        red_packet.expiry_time_changes < config::MAX_EXPIRY_TIME_CHANGES,
        RedPacketError::TooManyExpiryChanges
    );
    require!(
        expiry_time > current_time,
        RedPacketError::InvalidExpiryTime
    );
    require!(
        expiry_time <= current_time + (30 * 24 * 60 * 60),
        RedPacketError::InvalidExpiryTime
    );
    red_packet.expiry_time = expiry_time;
    red_packet.expiry_time_changes += 1;

    emit!(ExpiryTimeUpdated {
        red_packet: red_packet_key,
        new_expiry_time: expiry_time,
        red_packet_id: red_packet.red_packet_id,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct SetExpiryTime<'info> {
    #[account(mut)]
    pub red_packet: Account<'info, RedPacket>,
    pub authority: Signer<'info>,
}
