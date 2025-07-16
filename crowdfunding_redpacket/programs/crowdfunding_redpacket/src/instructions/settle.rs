use anchor_lang::prelude::*;

use crate::{
    constants::allocations::{CROWDFUNDING_NAME, LIQUIDITY_NAME, SETTLED_SOL_PERCENTAGES},
    errors::RedPacketError,
    events::CrowdfundingSettled,
    state::SettleCrowdfunding,
};

/*
   settle_crowdfunding - 结算众筹
   众筹结束后，根据众筹结果进行结算
   结算逻辑：
    1. 如果众筹成功，将众筹金额分配给项目方、开发者、协议方，并创建流动性池
    2. 如果众筹失败，标记状态，允许用户退款
    前置条件检查 -> 标记结算状态 -> 根据成功/失败分别处理 -> 发出事件
*/
pub fn handler(ctx: Context<SettleCrowdfunding>) -> Result<()> {
    let red_packet = &mut ctx.accounts.red_packet;
    let clock = Clock::get()?;

    // 验证状态
    require!(!red_packet.settled, RedPacketError::AlreadySettled);
    require!(
        clock.unix_timestamp >= red_packet.expiry_time,
        RedPacketError::CrowdfundingNotEnded
    );
    require!(red_packet.sol_raised > 0, RedPacketError::NoFundsRaised);
    require!(
        red_packet.funding_goal > 0,
        RedPacketError::InvalidFundingGoal
    );

    // 标记结算状态
    red_packet.settled = true;
    red_packet.success = red_packet.sol_raised >= red_packet.funding_goal;

    // 处理众筹成功的情况
    if red_packet.success {
        red_packet.unlock_start_time = clock.unix_timestamp; // 设置全局解锁开始时间
        red_packet.dev_fund_start_time = clock.unix_timestamp; // 设置开发者基金解锁开始时间

        let total_raised = red_packet.sol_raised;

        // 2. 解决 SOL 分配的舍入误差 (余额分配法)
        let mut remaining_sol = total_raised;

        let liquidity_sol = total_raised
            .checked_mul(SETTLED_SOL_PERCENTAGES.liquidity)
            .ok_or(RedPacketError::ArithmeticOverflow)?
            .checked_div(100)
            .ok_or(RedPacketError::ArithmeticOverflow)?;
        red_packet.liquidity_sol_amount = liquidity_sol;
        remaining_sol = remaining_sol
            .checked_sub(liquidity_sol)
            .ok_or(RedPacketError::ArithmeticOverflow)?;

        let dev_fund_sol = total_raised
            .checked_mul(SETTLED_SOL_PERCENTAGES.dev_fund)
            .ok_or(RedPacketError::ArithmeticOverflow)?
            .checked_div(100)
            .ok_or(RedPacketError::ArithmeticOverflow)?;
        red_packet.dev_fund_sol_amount = dev_fund_sol;
        remaining_sol = remaining_sol
            .checked_sub(dev_fund_sol)
            .ok_or(RedPacketError::ArithmeticOverflow)?;

        let protocol_fee = total_raised
            .checked_mul(SETTLED_SOL_PERCENTAGES.protocol)
            .ok_or(RedPacketError::ArithmeticOverflow)?
            .checked_div(100)
            .ok_or(RedPacketError::ArithmeticOverflow)?;
        red_packet.protocol_fee_amount = protocol_fee;
        remaining_sol = remaining_sol
            .checked_sub(protocol_fee)
            .ok_or(RedPacketError::ArithmeticOverflow)?;

        // 将所有剩余的 SOL 分配给创建者，确保总和精确
        red_packet.creator_direct_amount = remaining_sol;

        // 3. 计算并存储代币兑换率 (为 claim_tokens 做准备)
        // 这个兑换率告诉我们，每 1 lamport 的 SOL 可以换多少项目代币的最小单位
        const PRECISION: u128 = 1_000_000_000; // 使用一个精度因子来处理小数

        let crowdfunding_allocation = red_packet
            .allocations
            .iter()
            .find(|a| a.name == CROWDFUNDING_NAME) // 找到用于众筹奖励的代币池
            .ok_or(RedPacketError::MissingCrowdfundingAllocation)?;

        let total_crowdfunding_tokens = crowdfunding_allocation.amount;
        require!(
            total_crowdfunding_tokens > 0,
            RedPacketError::InvalidAllocationAmount
        );

        // 计算兑换率
        // 汇率计算精度：在计算 tokens_per_sol 时，您引入了一个 PRECISION 因子。
        // 这是一种处理链上定点数运算的标准方法，可以有效地保留小数部分的精度，使得后续用户领取代币时的计算更加准确。
        red_packet.tokens_per_sol = (total_crowdfunding_tokens as u128)
            .checked_mul(PRECISION)
            .ok_or(RedPacketError::ArithmeticOverflow)?
            .checked_div(total_raised as u128)
            .ok_or(RedPacketError::ArithmeticOverflow)?;

        let liquidity_allocation = red_packet
            .allocations
            .iter()
            .find(|a| a.name == LIQUIDITY_NAME)
            .ok_or(RedPacketError::MissingLiquidityAllocation)?;
        red_packet.liquidity_token_amount = liquidity_allocation.amount;

        if red_packet.creator_direct_amount > 0 {
            **ctx
                .accounts
                .sol_vault
                .to_account_info()
                .try_borrow_mut_lamports()? -= red_packet.creator_direct_amount;
            **ctx
                .accounts
                .creator
                .to_account_info()
                .try_borrow_mut_lamports()? += red_packet.creator_direct_amount;
        }

        // 初始化流动性池
        // let liquidity_pool = initialize_pool_with_liquidity(
        //     ctx,
        //     red_packet.liquidity_sol_amount,
        //     red_packet.liquidity_token_amount,
        // )?;
        // red_packet.liquidity_pool = liquidity_pool;
    }

    // 发出事件
    emit!(CrowdfundingSettled {
        red_packet: red_packet.key(),
        success: red_packet.success,
        sol_raised: red_packet.sol_raised,
        liquidity_sol_amount: red_packet.liquidity_sol_amount,
        liquidity_token_amount: red_packet.liquidity_token_amount,
        dev_fund_sol_amount: red_packet.dev_fund_sol_amount,
        creator_direct_amount: red_packet.creator_direct_amount,
        protocol_fee_amount: red_packet.protocol_fee_amount,
        liquidity_pool: red_packet.liquidity_pool,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

/**
 * 交易大小和原子性：Raydium 的创建池子、添加流动性等操作通常需要多个 CPI 调用。
 * 您需要确保将 settle_crowdfunding 和 Raydium 的相关操作放在同一个交易（Transaction）中。
 * 如果指令太多导致超出交易大小限制，就需要考虑将结算过程拆分成多个独立的、需要权限验证的指令（比如 settle_step_1_distribute_funds 和 settle_step_2_create_pool）。
 * 但目前来看，放在一起是最佳选择，以保证原子性。
 *
 * 权限和账户：调用 Raydium 的 CPI 需要传入大量的账户，包括 Raydium 的程序 ID、AMM 池的各种地址、Vault 地址等。
 * 在 SettleCrowdfunding 这个 Context 结构体中，需要提前定义好所有需要传入的 AccountInfo。
 *
 *  错误处理：与 Raydium 的交互可能会失败（例如，滑点过高、Raydium 协议升级等）。需要妥善处理来自 CPI 调用的错误。
 */
// 占位函数，待实现 Raydium 集成
#[allow(unused)]
fn initialize_pool_with_liquidity(
    ctx: Context<SettleCrowdfunding>,
    _sol_amount: u64,
    _token_amount: u64,
) -> Result<Pubkey> {
    // TODO: 调用 Raydium CPI 创建流动性池并添加流动性
    // 返回流动性池地址

    // 初始化 AMM 池
    // 调用 Raydium create_pool CPI
    // 转账 SOL 和代币到池 vault
    // 调用 Raydium add_liquidity CPI
    // 返回池地址
    Err(RedPacketError::RaydiumNotImplemented.into())
}
