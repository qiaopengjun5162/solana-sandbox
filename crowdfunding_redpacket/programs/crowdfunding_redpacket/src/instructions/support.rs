use anchor_lang::{
    prelude::*,
    system_program::{self, transfer},
};

use crate::{
    constants::support_tiers::{LARGE_SUPPORT_AMOUNT, MIN_SUPPORT_THRESHOLD, SMALL_SUPPORT_AMOUNT},
    errors::RedPacketError,
    events::CrowdfundingSupported,
    state::{SupportCrowdfunding, UnlockSchemeType},
};

/*
   support_crowdfunding - 支持众筹
   允许用户支持众筹项目
   支持金额：
    0.05 SOL（小额支持，立即解锁）
    0.5 SOL（大额支持，渐进解锁）
   特点：
    大额支持有更长的解锁期
    小额支持立即解锁
*/
pub fn handler(ctx: Context<SupportCrowdfunding>, amount: u64) -> Result<()> {
    let red_packet = &mut ctx.accounts.red_packet;
    let backer_state = &mut ctx.accounts.backer_state;
    let clock = Clock::get()?;

    // 验证红包状态
    require!(
        clock.unix_timestamp < red_packet.expiry_time,
        RedPacketError::CrowdfundingEnded
    );
    require!(!red_packet.settled, RedPacketError::RedPacketSettled);
    require!(backer_state.amount == 0, RedPacketError::AlreadySupported);

    let remaining_goal = red_packet
        .funding_goal
        .saturating_sub(red_packet.sol_raised);
    if remaining_goal < MIN_SUPPORT_THRESHOLD {
        // 如果剩余目标很少，只允许小额支持
        require!(
            amount == SMALL_SUPPORT_AMOUNT,
            RedPacketError::InvalidSupportAmount
        );
    } else {
        // 否则，允许大额或小额支持
        require!(
            amount == SMALL_SUPPORT_AMOUNT || amount == LARGE_SUPPORT_AMOUNT,
            RedPacketError::InvalidSupportAmount
        );
    }

    // 执行 SOL 转账到 sol_vault
    let transfer_instruction = system_program::Transfer {
        from: ctx.accounts.backer.to_account_info(),
        to: ctx.accounts.sol_vault.to_account_info(),
    };

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction,
        ),
        amount,
    )?;

    // 更新红包状态
    red_packet.sol_raised = red_packet
        .sol_raised
        .checked_add(amount)
        .ok_or(RedPacketError::ArithmeticOverflow)?;

    // 设置 backer_state
    backer_state.amount = amount;
    backer_state.refunded = false;
    backer_state.claimed_amount = 0;

    // 根据支持金额设置解锁方案
    if amount == LARGE_SUPPORT_AMOUNT {
        backer_state.unlock_scheme = UnlockSchemeType::Gradual;
    } else {
        backer_state.unlock_scheme = UnlockSchemeType::Immediate;
    }

    // 发出事件
    emit!(CrowdfundingSupported {
        backer: ctx.accounts.backer.key(),
        red_packet: red_packet.key(),
        amount,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
