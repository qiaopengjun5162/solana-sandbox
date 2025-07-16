use anchor_lang::prelude::*;

use crate::{errors::RedPacketError, events::FeesDistributed, state::DistributeFees};

/*
   distribute_fees - 分配费用
   众筹成功时，由创建者触发，将协议费用分配给项目方和开发方
   分配逻辑：
    1. 从链上状态读取总费用，不再依赖外部参数。
    2. 检查费用是否已被分配，防止重复执行。
    3. 按比例计算项目方费用，剩余部分全部分配给开发方。
    4. 将费用转账给双方。
    5. 标记状态为“已分配”。
*/
pub fn handler(ctx: Context<DistributeFees>) -> Result<()> {
    let red_packet = &mut ctx.accounts.red_packet;
    let clock = Clock::get()?;

    // --- 1. 全新的、更严格的验证 ---
    require!(red_packet.success, RedPacketError::CrowdfundingFailed);
    require!(red_packet.settled, RedPacketError::CrowdfundingNotSettled);

    // 关键检查：防止重复分配费用
    require!(
        !red_packet.fees_distributed,
        RedPacketError::FeesAlreadyDistributed
    );

    // 从链上状态读取总费用
    let total_fee = red_packet.protocol_fee_amount;
    require!(total_fee > 0, RedPacketError::NoFeesToDistribute);

    require!(
        **ctx.accounts.sol_vault.to_account_info().lamports.borrow() >= total_fee,
        RedPacketError::InsufficientVaultBalance
    );

    // 验证百分比配置
    require!(
        red_packet.liquidity_fee_creator_percent <= 1000,
        RedPacketError::InvalidFeePercentage
    );

    // --- 2. 完整的费用计算逻辑 ---
    // 计算创建者应得的费用，并使用安全的 checked_div
    let creator_fee = total_fee
        .checked_mul(red_packet.liquidity_fee_creator_percent as u64)
        .ok_or(RedPacketError::ArithmeticOverflow)?
        .checked_div(1000)
        .ok_or(RedPacketError::ArithmeticOverflow)?;

    // 使用“余额分配法”，剩余的费用全部归开发者/平台，确保没有资金沉淀
    let developer_fee = total_fee.saturating_sub(creator_fee);

    // --- 4. 分别执行转账 ---

    // 转账 SOL 到 creator (如果费用大于0)
    if creator_fee > 0 {
        **ctx
            .accounts
            .sol_vault
            .to_account_info()
            .try_borrow_mut_lamports()? -= creator_fee;
        **ctx
            .accounts
            .creator
            .to_account_info()
            .try_borrow_mut_lamports()? += creator_fee;
    }

    // 转账 SOL 到 developer (如果费用大于0)
    if developer_fee > 0 {
        **ctx
            .accounts
            .sol_vault
            .to_account_info()
            .try_borrow_mut_lamports()? -= developer_fee;
        **ctx
            .accounts
            .developer_wallet
            .to_account_info()
            .try_borrow_mut_lamports()? += developer_fee;
    }

    // --- 5. 更新状态 ---
    // 标记费用已被分配
    red_packet.fees_distributed = true;

    // --- 6. 发出事件 ---
    emit!(FeesDistributed {
        red_packet: red_packet.key(),
        trigger: ctx.accounts.creator.key(), // 使用 creator 的 key 作为触发者
        total_distributed: total_fee,
        creator_fee: creator_fee,
        developer_fee: developer_fee,
        developer_wallet: ctx.accounts.developer_wallet.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
