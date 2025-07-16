use std::cmp;

use anchor_lang::prelude::*;
use anchor_spl::{token_2022::TransferChecked, token_interface};

use crate::{
    constants::{time::SECONDS_IN_A_DAY, vesting::LARGE_SUPPORT_UNLOCK_SCHEME},
    errors::RedPacketError,
    events::TokensClaimed,
    state::{ClaimTokens, UnlockSchemeType},
};

/*
   claim_tokens - 领取代币
   允许用户在众筹成功时领取代币
   领取逻辑：
    1. 如果众筹失败，不允许领取代币
    2. 如果众筹成功，根据支持者的解锁方案进行代币领取
    代币解锁机制：支持者和开发者的代币都会按时间逐步解锁
*/
pub fn handler(ctx: Context<ClaimTokens>) -> Result<()> {
    let backer_state = &mut ctx.accounts.backer_state;
    let red_packet = &ctx.accounts.red_packet;
    let clock = Clock::get()?;

    // --- 验证状态 ---
    require!(red_packet.success, RedPacketError::CrowdfundingFailed);
    require!(red_packet.settled, RedPacketError::CrowdfundingNotSettled);
    require!(backer_state.amount > 0, RedPacketError::NoContribution);
    require!(
        red_packet.unlock_start_time > 0,
        RedPacketError::UnlockNotStarted
    );

    // 1. 计算该用户总共应得的代币奖励
    // (backer_state.amount * red_packet.tokens_per_sol) / PRECISION
    const PRECISION: u128 = 1_000_000_000;
    let total_token_reward = (backer_state.amount as u128) // 用户支持的 SOL (lamports)
        .checked_mul(red_packet.tokens_per_sol)
        .ok_or(RedPacketError::ArithmeticOverflow)?
        .checked_div(PRECISION)
        .ok_or(RedPacketError::ArithmeticOverflow)? as u64;

    // 2. 根据解锁方案，计算到目前为止已解锁的总额度
    let total_claimable_to_date = match backer_state.unlock_scheme {
        UnlockSchemeType::Immediate => {
            // 立即解锁方案，可领取全部
            total_token_reward
        }
        UnlockSchemeType::Gradual => {
            // 渐进解锁方案
            let mut unlocked_percentage: u8 = 0;
            // 假设 LARGE_SUPPORT_UNLOCK_SCHEME = [(30, 20), (90, 30), ...]
            for &(offset_days, perc) in LARGE_SUPPORT_UNLOCK_SCHEME.iter() {
                let unlock_timestamp = red_packet
                    .unlock_start_time
                    .checked_add(offset_days as i64 * SECONDS_IN_A_DAY)
                    .ok_or(RedPacketError::ArithmeticOverflow)?;

                if clock.unix_timestamp >= unlock_timestamp {
                    // unlocked_percentage += perc;
                    unlocked_percentage = cmp::min(100, unlocked_percentage.saturating_add(perc));
                }
            }
            // 计算到目前为止可领取的总额
            (total_token_reward as u128)
                .checked_mul(unlocked_percentage as u128)
                .ok_or(RedPacketError::ArithmeticOverflow)?
                .checked_div(100)
                .ok_or(RedPacketError::ArithmeticOverflow)? as u64
        }
    };

    // 3. 计算本次可以领取的数量
    // 本次可领取的净额 to_claim = total_claimable_to_date - backer_state.claimed_amount
    let to_claim = total_claimable_to_date
        .checked_sub(backer_state.claimed_amount)
        .ok_or(RedPacketError::ArithmeticOverflow)?;

    require!(to_claim > 0, RedPacketError::NoVestedTokensToClaim);

    let creator_key = ctx.accounts.creator.key();
    let seeds = &[b"red_packet", creator_key.as_ref(), &[ctx.bumps.red_packet]];
    let signer_seeds = &[&seeds[..]];

    token_interface::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.token_vault.to_account_info(),
                to: ctx.accounts.claimer_ata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.red_packet.to_account_info(),
            },
            signer_seeds,
        ),
        to_claim,
        ctx.accounts.mint.decimals,
    )?;

    // --- 更新状态 (使用安全的 checked_add) ---
    // 在转账成功后，立即并安全地更新了 backer_state.claimed_amount，防止了用户重复领取同一部分的代币
    backer_state.claimed_amount = backer_state
        .claimed_amount
        .checked_add(to_claim)
        .ok_or(RedPacketError::ArithmeticOverflow)?;

    // --- 发出事件 ---
    emit!(TokensClaimed {
        backer: ctx.accounts.claimer.key(),
        red_packet: red_packet.key(),
        amount: to_claim,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
