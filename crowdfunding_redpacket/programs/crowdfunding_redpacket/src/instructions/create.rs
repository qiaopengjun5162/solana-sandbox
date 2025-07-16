use std::collections::HashSet;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, TransferChecked};

use crate::{
    constants::{
        airdrop::DEFAULT_MAX_COUNT,
        allocations::{
            AIRDROP_NAME, CROWDFUNDING_NAME, DEFAULT_TOKEN_PERCENTAGES, DEVELOPER_NAME,
            LIQUIDITY_NAME,
        },
        config::MAX_ALLOCATION_COUNT,
    },
    errors::RedPacketError,
    events::RedPacketCreated,
    state::{AllocationEntry, CreateCustomRedpacket, CustomCrowdfundingParams},
};

/*
   create_custom_redpacket - 创建自定义红包
   初始化一个新的众筹红包项目
   自定义分配众筹：创建者可以为不同用途（空投、众筹、流动性、开发等）设置代币分配
*/
pub fn handler(
    ctx: Context<CreateCustomRedpacket>,
    params: CustomCrowdfundingParams,
) -> Result<()> {
    let red_packet = &mut ctx.accounts.red_packet;
    let creator = &ctx.accounts.creator;
    let clock = Clock::get()?;

    // 输入验证
    require!(params.total_amount > 0, RedPacketError::InvalidTotalAmount);
    require!(
        !params.token_name.is_empty() && params.token_name.len() <= 32,
        RedPacketError::InvalidTokenName
    );
    require!(
        !params.token_symbol.is_empty() && params.token_symbol.len() <= 10,
        RedPacketError::InvalidTokenSymbol
    );
    require!(params.funding_goal > 0, RedPacketError::InvalidFundingGoal);
    require!(
        params.allocations.len() <= MAX_ALLOCATION_COUNT,
        RedPacketError::TooManyAllocationTypes
    );

    // 验证 mint 账户
    require!(
        ctx.accounts.mint.is_initialized,
        RedPacketError::InvalidMint
    );

    // 设置过期时间
    let expiry_duration = params.expiry_duration.unwrap_or(14 * 24 * 60 * 60);
    let expiry_time = clock
        .unix_timestamp
        .checked_add(expiry_duration)
        .ok_or(RedPacketError::ArithmeticOverflow)?;
    require!(
        expiry_time > clock.unix_timestamp,
        RedPacketError::InvalidExpiryTime
    );

    // 设置分配比例
    let allocations = if params.allocations.is_empty() {
        // 在默认分配逻辑中
        let mut allocations = Vec::new();
        let mut remaining_amount = params.total_amount;

        // 计算 airdrop
        let airdrop_amount = params
            .total_amount
            .checked_mul(DEFAULT_TOKEN_PERCENTAGES.airdrop)
            .ok_or(RedPacketError::ArithmeticOverflow)?
            .checked_div(100)
            .ok_or(RedPacketError::ArithmeticOverflow)?;
        allocations.push(AllocationEntry {
            name: AIRDROP_NAME.to_string(),
            amount: airdrop_amount,
            unlock_months: 12,
        });
        remaining_amount = remaining_amount
            .checked_sub(airdrop_amount)
            .ok_or(RedPacketError::ArithmeticOverflow)?;

        // 计算 crowdfunding
        let crowdfunding_amount = params
            .total_amount
            .checked_mul(DEFAULT_TOKEN_PERCENTAGES.crowdfunding)
            .ok_or(RedPacketError::ArithmeticOverflow)?
            .checked_div(100)
            .ok_or(RedPacketError::ArithmeticOverflow)?;
        allocations.push(AllocationEntry {
            name: CROWDFUNDING_NAME.to_string(),
            amount: crowdfunding_amount,
            unlock_months: 12,
        });
        remaining_amount = remaining_amount
            .checked_sub(crowdfunding_amount)
            .ok_or(RedPacketError::ArithmeticOverflow)?;

        // 计算 liquidity
        let liquidity_amount = params
            .total_amount
            .checked_mul(DEFAULT_TOKEN_PERCENTAGES.liquidity)
            .ok_or(RedPacketError::ArithmeticOverflow)?
            .checked_div(100)
            .ok_or(RedPacketError::ArithmeticOverflow)?;
        allocations.push(AllocationEntry {
            name: LIQUIDITY_NAME.to_string(),
            amount: liquidity_amount,
            unlock_months: 0,
        });
        remaining_amount = remaining_amount
            .checked_sub(liquidity_amount)
            .ok_or(RedPacketError::ArithmeticOverflow)?;

        // 将所有剩余的金额分配给 developer，以避免舍入误差 “余额分配法”
        allocations.push(AllocationEntry {
            name: DEVELOPER_NAME.to_string(),
            amount: remaining_amount,
            unlock_months: 12,
        });

        allocations
    } else {
        let mut names = HashSet::new();
        // 验证自定义分配
        for alloc in &params.allocations {
            require!(alloc.amount > 0, RedPacketError::InvalidAllocationAmount);
            require!(
                !alloc.name.is_empty() && alloc.name.len() <= 32,
                RedPacketError::InvalidAllocationName
            );
            require!(
                names.insert(alloc.name.clone()),
                RedPacketError::DuplicateAllocationName
            );
        }
        let total_alloc: u64 = params.allocations.iter().map(|a| a.amount).sum();
        require!(
            total_alloc == params.total_amount,
            RedPacketError::InvalidAllocation
        );
        // 验证空投和流动性分配存在
        require!(
            params.allocations.iter().any(|a| a.name == AIRDROP_NAME),
            RedPacketError::MissingAirdropAllocation
        );
        require!(
            params.allocations.iter().any(|a| a.name == LIQUIDITY_NAME),
            RedPacketError::MissingLiquidityAllocation
        );
        params.allocations
    };

    // 设置空投最大数量
    let airdrop_max_count = params.airdrop_max_count.unwrap_or(DEFAULT_MAX_COUNT);
    require!(
        airdrop_max_count > 0,
        RedPacketError::InvalidAirdropMaxCount
    );

    // 从 allocations 中找到空投分配
    let airdrop_alloc = allocations
        .iter()
        .find(|a| a.name == AIRDROP_NAME)
        .ok_or(RedPacketError::MissingAirdropAllocation)?; // 确保空投分配存在

    // 防止空投总额小于最大人数
    require!(
        airdrop_alloc.amount >= airdrop_max_count as u64,
        RedPacketError::AirdropAmountTooLowForMaxCount
    );

    token_interface::transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.creator_token_account.to_account_info(),
                to: ctx.accounts.token_vault.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: creator.to_account_info(), // authority 是交易的签名者 creator
            },
        ),
        params.total_amount,
        ctx.accounts.mint.decimals,
    )?;

    // 初始化 red_packet 账户
    red_packet.creator = creator.key();
    red_packet.mint = params.mint;
    red_packet.total_amount = params.total_amount;
    red_packet.token_name = params.token_name.clone();
    red_packet.token_symbol = params.token_symbol.clone();
    red_packet.funding_goal = params.funding_goal;
    red_packet.allocations = allocations;
    red_packet.sol_raised = 0;
    red_packet.expiry_time = expiry_time;
    red_packet.settled = false;
    red_packet.success = false;
    red_packet.airdrop_max_count = airdrop_max_count;
    red_packet.airdrop_claimed = 0;

    red_packet.liquidity_pool = Pubkey::default();
    red_packet.dev_fund_claimed = 0;
    red_packet.dev_fund_start_time = 0;
    red_packet.protocol_fee_amount = 0;
    red_packet.creator_direct_amount = 0;
    red_packet.liquidity_sol_amount = 0;
    red_packet.liquidity_token_amount = 0;
    red_packet.liquidity_fee_creator_percent = 1;

    // 触发创建事件
    emit!(RedPacketCreated {
        creator: creator.key(),
        red_packet: red_packet.key(),
        name: params.token_name,
        symbol: params.token_symbol,
        funding_goal: params.funding_goal,
        expiry_time: expiry_time,
        total_supply: params.total_amount,
        allocations: red_packet.allocations.clone(),
        timestamp: clock.unix_timestamp
    });

    Ok(())
}
