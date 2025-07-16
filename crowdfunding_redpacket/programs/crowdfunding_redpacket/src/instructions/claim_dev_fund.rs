use anchor_lang::prelude::*;

use crate::{
    constants::{allocations::DEVELOPER_NAME, time::SECONDS_PER_MONTH},
    errors::RedPacketError,
    events::DevFundClaimed,
    state::ClaimDevFund,
};

/*
   claim_dev_fund - 领取开发基金
   允许项目方在众筹成功时按月解锁领取开发基金
   领取逻辑：
    1. 如果众筹失败，不允许领取开发基金
    2. 如果众筹成功，根据解锁方案（按月解锁）领取已解锁部分
*/
pub fn handler(ctx: Context<ClaimDevFund>) -> Result<()> {
    let red_packet = &mut ctx.accounts.red_packet;
    let clock = Clock::get()?.unix_timestamp;

    // 验证创建者
    require!(
        red_packet.creator == ctx.accounts.creator.key(),
        RedPacketError::InvalidCreator
    );

    // 验证众筹状态
    require!(red_packet.success, RedPacketError::CrowdfundingFailed);
    require!(red_packet.settled, RedPacketError::CrowdfundingNotSettled);
    require!(
        red_packet.dev_fund_start_time > 0,
        RedPacketError::DevFundNotStarted
    );

    // 验证解锁开始时间
    require!(
        clock >= red_packet.dev_fund_start_time,
        RedPacketError::DevFundNotStarted
    );

    // 1. 在 allocations 数组中找到开发者基金的条目
    let dev_fund_entry = red_packet
        .allocations
        .iter()
        .find(|a| a.name == DEVELOPER_NAME)
        .ok_or(RedPacketError::MissingDeveloperAllocation)?; // 确保开发者分配存在

    let dev_fund_total_amount = dev_fund_entry.amount;
    let dev_fund_unlock_months = dev_fund_entry.unlock_months as u64;

    require!(
        dev_fund_total_amount > 0,
        RedPacketError::NoDevFundAllocated
    );
    require!(
        dev_fund_unlock_months > 0,
        RedPacketError::InvalidUnlockMonths
    );

    // 2. 使用从条目中获取的数据进行计算
    // 计算已经过的月份
    let elapsed = ((clock - red_packet.dev_fund_start_time) / SECONDS_PER_MONTH) as u64;
    // 计算已解锁的月份
    let unlocked_months = elapsed.min(dev_fund_unlock_months);
    // 计算已解锁的总金额 unlocked_amount = dev_fund_amount * unlocked_months / dev_fund_unlock_months
    let unlocked_amount = dev_fund_total_amount
        .checked_mul(unlocked_months)
        .ok_or(RedPacketError::ArithmeticOverflow)?
        .checked_div(dev_fund_unlock_months as u64)
        .ok_or(RedPacketError::ArithmeticOverflow)?;
    // 计算本次可领取的金额
    let claimable = unlocked_amount.saturating_sub(red_packet.dev_fund_claimed);
    require!(claimable > 0, RedPacketError::NoDevFundToClaim);

    // 验证 sol_vault 余额
    require!(
        **ctx.accounts.sol_vault.to_account_info().lamports.borrow() >= claimable,
        RedPacketError::InsufficientVaultBalance
    );

    **ctx
        .accounts
        .sol_vault
        .to_account_info()
        .try_borrow_mut_lamports()? -= claimable;
    **ctx
        .accounts
        .creator
        .to_account_info()
        .try_borrow_mut_lamports()? += claimable;

    // 更新状态
    red_packet.dev_fund_claimed = red_packet
        .dev_fund_claimed
        .checked_add(claimable)
        .ok_or(RedPacketError::ArithmeticOverflow)?;

    // 发出事件
    emit!(DevFundClaimed {
        creator: ctx.accounts.creator.key(),
        red_packet: red_packet.key(),
        amount: claimable,
        timestamp: clock,
    });

    Ok(())
}
