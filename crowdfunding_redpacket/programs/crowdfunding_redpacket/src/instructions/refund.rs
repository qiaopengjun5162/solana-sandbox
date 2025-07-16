use anchor_lang::prelude::*;

use crate::{
    constants::time::REFUND_WINDOW_SECS, errors::RedPacketError, events::Refunded, state::Refund,
};

/*
   refund - 退款
   允许用户在众筹失败时进行退款
   退款逻辑：
    1. 如果众筹成功，不允许退款
    2. 如果众筹失败，允许用户在指定时间内进行退款
    // 退款机制：如果未达到众筹目标，支持者可以获得退款
*/
pub fn handler(ctx: Context<Refund>) -> Result<()> {
    let red_packet = &ctx.accounts.red_packet;
    let backer_state = &mut ctx.accounts.backer_state;
    let now = Clock::get()?.unix_timestamp;

    // 验证状态
    require!(red_packet.settled, RedPacketError::CrowdfundingNotEnded);
    require!(!red_packet.success, RedPacketError::NotRefundable);
    require!(!backer_state.refunded, RedPacketError::AlreadyRefunded);
    require!(
        now <= red_packet.expiry_time + REFUND_WINDOW_SECS,
        RedPacketError::RefundWindowClosed
    );
    // 1. 将要退款的金额存储到临时变量中
    let refund_amount = backer_state.amount;
    require!(refund_amount > 0, RedPacketError::NoContribution);

    // 验证 sol_vault 余额
    require!(
        **ctx.accounts.sol_vault.to_account_info().lamports.borrow() >= refund_amount,
        RedPacketError::InsufficientVaultBalance
    );

    **ctx
        .accounts
        .sol_vault
        .to_account_info()
        .try_borrow_mut_lamports()? -= refund_amount;
    **ctx
        .accounts
        .backer
        .to_account_info()
        .try_borrow_mut_lamports()? += refund_amount;

    // 更新状态
    backer_state.refunded = true;
    backer_state.amount = 0;

    // 发出事件
    emit!(Refunded {
        backer: ctx.accounts.backer.key(),
        red_packet: red_packet.key(),
        amount: refund_amount,
        timestamp: now,
    });

    Ok(())
}
