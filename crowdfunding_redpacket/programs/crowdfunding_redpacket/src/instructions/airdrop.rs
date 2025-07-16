use anchor_lang::prelude::*;
use anchor_spl::{token_2022::TransferChecked, token_interface};

use crate::{
    constants::allocations::AIRDROP_NAME, errors::RedPacketError, events::AirdropClaimed,
    state::ClaimAirdrop,
};

/*
   claim_airdrop - 领取空投
   允许用户领取空投代币
*/
pub fn handler(ctx: Context<ClaimAirdrop>) -> Result<()> {
    let red_packet = &mut ctx.accounts.red_packet;
    let airdrop_state = &mut ctx.accounts.airdrop_state;
    let clock = Clock::get()?;

    // 验证红包状态
    require!(
        clock.unix_timestamp < red_packet.expiry_time,
        RedPacketError::CrowdfundingEnded
    );
    require!(!red_packet.settled, RedPacketError::RedPacketSettled);

    // 检查用户是否已经领取过
    require!(!airdrop_state.claimed, RedPacketError::AlreadyClaimed);

    // 检查空投是否还有剩余
    require!(
        red_packet.airdrop_claimed < red_packet.airdrop_max_count,
        RedPacketError::AirdropExhausted
    );

    // 获取空投分配
    let airdrop_allocation = red_packet
        .allocations
        .iter()
        .find(|a| a.name == AIRDROP_NAME)
        .ok_or(RedPacketError::MissingAirdropAllocation)?;

    // 计算每用户空投金额
    require!(
        red_packet.airdrop_max_count > 0,
        RedPacketError::InvalidAirdropMaxCount
    );

    let per_user_amount = if red_packet.airdrop_claimed + 1 == red_packet.airdrop_max_count {
        // 这是最后一位领取者，领取所有剩余的空投代币
        let total_claimed_so_far = airdrop_allocation
            .amount
            .checked_div(red_packet.airdrop_max_count as u64)
            .ok_or(RedPacketError::ArithmeticOverflow)?
            .checked_mul(red_packet.airdrop_claimed as u64)
            .ok_or(RedPacketError::ArithmeticOverflow)?;

        // 用总空投量，减去之前已经分配掉的量，剩下的就是最后一个人应该拿的
        airdrop_allocation
            .amount
            .checked_sub(total_claimed_so_far)
            .ok_or(RedPacketError::ArithmeticOverflow)?
    } else {
        // 非最后一位领取者，按平均值领取
        airdrop_allocation
            .amount
            .checked_div(red_packet.airdrop_max_count as u64)
            .ok_or(RedPacketError::ArithmeticOverflow)?
    };
    require!(
        per_user_amount > 0,
        RedPacketError::AirdropAmountTooLowForMaxCount
    );

    // 准备 PDA 签名种子
    let authority = red_packet.to_account_info();
    let seeds = &[
        b"red_packet",
        ctx.accounts.creator.key.as_ref(),
        &[ctx.bumps.red_packet],
    ];
    let signer_seeds = &[&seeds[..]];

    // 将代币从 token_vault 转账到用户钱包
    token_interface::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.token_vault.to_account_info(),
                to: ctx.accounts.claimer_ata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority,
            },
            signer_seeds,
        ),
        per_user_amount,
        ctx.accounts.mint.decimals,
    )?;

    // 更新状态
    airdrop_state.claimed = true;
    red_packet.airdrop_claimed = red_packet
        .airdrop_claimed
        .checked_add(1)
        .ok_or(RedPacketError::ArithmeticOverflow)?;

    // 触发事件
    emit!(AirdropClaimed {
        claimer: ctx.accounts.claimer.key(),
        red_packet: red_packet.key(),
        amount: per_user_amount,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
