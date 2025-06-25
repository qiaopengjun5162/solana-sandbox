#![allow(unexpected_cfgs)]

use crate::{
    config::{
        self, CREATE_FEE, DEFAULT_RED_PACKET_EXPIRY_DAYS, FEE_RECEIVER, MAX_PACKET_COUNT,
        RED_PACKET_SPACE,
    },
    errors::RedPacketError,
    events::RedPacketCreated,
    CreatorState, RedPacket,
};
use anchor_lang::solana_program::program_option::COption as SolanaCOption;
use anchor_lang::{prelude::*, solana_program::program_pack::Pack, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::spl_token,
    token_2022::TransferChecked,
    token_interface::{self, TokenInterface},
};
use spl_token::state::Mint as MintLegacy;
use spl_token_2022::extension::transfer_fee::TransferFeeConfig;
use spl_token_2022::{
    extension::{
        confidential_transfer::ConfidentialTransferMint, non_transferable::NonTransferable,
        permanent_delegate::PermanentDelegate, transfer_hook::TransferHook,
        BaseStateWithExtensions, StateWithExtensions,
    },
    state::Mint as Mint2022,
}; // 引入 SPL Token 的 Mint 结构

pub fn create_handler(
    ctx: Context<CreateRedPacket>,
    total_amount: u64,
    packet_count: u32,
    red_packet_type: u8,
    merkle_root: Option<[u8; 32]>,
    is_sol: bool,
    expiry_days: Option<i64>,
    random_seed: Option<u64>,
) -> Result<()> {
    validate_common_parameters(packet_count, total_amount, expiry_days)?;

    let red_packet_id = initialize_red_packet(
        &mut ctx.accounts.creator_state,
        &mut ctx.accounts.red_packet,
        &ctx.bumps.red_packet,
        expiry_days,
        red_packet_type,
        random_seed,
        merkle_root,
        ctx.accounts.creator.key(),
        ctx.accounts.mint.key(),
        total_amount,
        packet_count,
        is_sol,
    )?;

    let (has_transfer_fee, has_transfer_hook, has_permanent_delegate, has_close_authority);
    if is_sol {
        let rent_exempt = Rent::get()?.minimum_balance(RED_PACKET_SPACE);
        require!(
            ctx.accounts.creator.lamports() >= total_amount + rent_exempt + CREATE_FEE,
            RedPacketError::InsufficientFunds
        );

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.creator.to_account_info(),
                    to: ctx.accounts.red_packet.to_account_info(),
                },
            ),
            total_amount,
        )?;

        has_transfer_fee = false;
        has_transfer_hook = false;
        has_permanent_delegate = false;
        has_close_authority = false;
    } else {
        if ctx.accounts.pool_ata.data_is_empty() {
            anchor_spl::associated_token::create(CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: ctx.accounts.creator.to_account_info(),
                    associated_token: ctx.accounts.pool_ata.to_account_info(),
                    authority: ctx.accounts.red_packet.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
            ))?;
        }

        let mint_data = ctx.accounts.mint.data.borrow();

        let decimals;
        let actual_amount;
        if ctx.accounts.token_program.key() == spl_token::id() {
            // 处理 SPL Token
            let mint =
                MintLegacy::unpack(&mint_data).map_err(|_| RedPacketError::InvalidMintAccount)?;
            decimals = mint.decimals;
            actual_amount = total_amount; // SPL Token 无转账费用
            has_transfer_fee = false;
            has_transfer_hook = false;
            has_permanent_delegate = false;
            has_close_authority = false;
        } else if ctx.accounts.token_program.key() == spl_token_2022::id() {
            // 处理 Token-2022
            let mint = StateWithExtensions::<Mint2022>::unpack(&mint_data)
                .map_err(|_| RedPacketError::InvalidMintAccount)?;
            decimals = mint.base.decimals;

            // 检查扩展特性
            let transfer_fee = mint.get_extension::<TransferFeeConfig>().is_ok();
            let transfer_hook = mint.get_extension::<TransferHook>().is_ok();
            let permanent_delegate = mint.get_extension::<PermanentDelegate>().is_ok();
            let confidential_transfer = mint.get_extension::<ConfidentialTransferMint>().is_ok();
            let non_transferable = mint.get_extension::<NonTransferable>().is_ok();
            let close_authority = mint.base.freeze_authority != SolanaCOption::None;

            // 禁用不支持的扩展
            require!(!transfer_hook, RedPacketError::TransferHookNotSupported);
            require!(
                !permanent_delegate,
                RedPacketError::PermanentDelegateNotSupported
            );
            require!(
                !confidential_transfer,
                RedPacketError::ConfidentialTransferNotSupported
            );
            require!(
                !non_transferable,
                RedPacketError::NonTransferableNotSupported
            );

            actual_amount = if transfer_fee {
                let fee_config = mint
                    .get_extension::<TransferFeeConfig>()
                    .map_err(|_| RedPacketError::InvalidExtension)?;
                // Properly convert PodU64 to u64
                let epoch = u64::from(fee_config.newer_transfer_fee.epoch);
                let post_fee_amount = total_amount;

                let fee = fee_config
                    .calculate_inverse_epoch_fee(epoch, post_fee_amount)
                    .ok_or(RedPacketError::FeeCalculationError)?;
                total_amount
                    .checked_add(fee)
                    .ok_or(RedPacketError::ArithmeticOverflow)?
            } else {
                total_amount
            };

            has_transfer_fee = transfer_fee;
            has_transfer_hook = transfer_hook;
            has_permanent_delegate = permanent_delegate;
            has_close_authority = close_authority;
        } else {
            return Err(RedPacketError::InvalidTokenProgram.into());
        };

        // 转移SPL代币到pool_ata
        token_interface::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.creator_ata.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.pool_ata.to_account_info(),
                    authority: ctx.accounts.creator.to_account_info(),
                },
            ),
            actual_amount,
            decimals,
        )?;
    }

    // 更新事件
    finalize_creation(
        &ctx,
        red_packet_id,
        has_transfer_fee,
        has_transfer_hook,
        has_permanent_delegate,
        has_close_authority,
    )
}

/// 初始化红包账户数据
fn initialize_red_packet(
    creator_state: &mut Account<'_, CreatorState>,
    red_packet: &mut Account<'_, RedPacket>,
    bump: &u8,
    expiry_days: Option<i64>,
    red_packet_type: u8,
    random_seed: Option<u64>,
    merkle_root: Option<[u8; 32]>,
    creator_key: Pubkey,
    mint_key: Pubkey,
    total_amount: u64,
    packet_count: u32,
    is_sol: bool,
) -> Result<u64> {
    let clock = Clock::get()?;
    let expiry_time = clock.unix_timestamp
        + (expiry_days.unwrap_or(DEFAULT_RED_PACKET_EXPIRY_DAYS) * 24 * 60 * 60);
    let red_packet_id = creator_state.next_red_packet_id;

    creator_state.next_red_packet_id = red_packet_id
        .checked_add(1)
        .ok_or(RedPacketError::CounterOverflow)?;

    let (share_amount, random_seed_val, merkle_root_val) = match red_packet_type {
        0 => {
            let share = total_amount / packet_count as u64;
            require!(share > 0, RedPacketError::InvalidShareAmount);
            (share, 0, [0; 32])
        }
        1 => (
            0,
            random_seed.ok_or(RedPacketError::RandomSeedRequired)?,
            [0; 32],
        ),
        2 => (0, 0, merkle_root.ok_or(RedPacketError::InvalidMerkleRoot)?),
        _ => return Err(RedPacketError::InvalidRedPacketType.into()),
    };

    red_packet.set_inner(RedPacket {
        creator: creator_key,
        mint: mint_key,
        total_amount,
        remaining_amount: total_amount,
        packet_count,
        claimed_count: 0,
        red_packet_type,
        expiry_time,
        is_sol,
        expiry_time_changes: 0,
        red_packet_id,
        bump: *bump,
        share_amount,
        random_seed: random_seed_val,
        merkle_root: merkle_root_val,
    });

    Ok(red_packet_id)
}

fn validate_common_parameters(
    packet_count: u32,
    total_amount: u64,
    expiry_days: Option<i64>,
) -> Result<()> {
    require!(packet_count > 0, RedPacketError::InvalidPacketCount);
    require!(
        total_amount >= packet_count as u64,
        RedPacketError::InsufficientTotalAmount
    );
    require!(
        packet_count <= MAX_PACKET_COUNT,
        RedPacketError::PacketCountTooLarge
    );

    let expiry_days = expiry_days.unwrap_or(DEFAULT_RED_PACKET_EXPIRY_DAYS);
    require!(
        (1..=30).contains(&expiry_days),
        RedPacketError::InvalidExpiryDays
    );
    Ok(())
}

/// 最终处理：支付费用并发送事件
fn finalize_creation(
    ctx: &Context<CreateRedPacket>,
    red_packet_id: u64,
    has_transfer_fee: bool,
    has_transfer_hook: bool,
    has_permanent_delegate: bool,
    has_close_authority: bool,
) -> Result<()> {
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.creator.to_account_info(),
                to: ctx.accounts.fee_receiver.to_account_info(),
            },
        ),
        CREATE_FEE,
    )?;

    emit!(RedPacketCreated {
        creator: ctx.accounts.creator.key(),
        red_packet: ctx.accounts.red_packet.key(),
        total_amount: ctx.accounts.red_packet.total_amount,
        packet_count: ctx.accounts.red_packet.packet_count,
        red_packet_type: ctx.accounts.red_packet.red_packet_type,
        expiry_time: ctx.accounts.red_packet.expiry_time,
        is_sol: ctx.accounts.red_packet.is_sol,
        red_packet_id,
        bump: ctx.accounts.red_packet.bump,
        mint: ctx.accounts.red_packet.mint,
        has_transfer_fee,
        has_transfer_hook,
        has_permanent_delegate,
        has_close_authority,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(total_amount: u64, packet_count: u32, red_packet_type: u8, merkle_root: Option<[u8; 32]>, is_sol: bool, expiry_days: Option<i64>, random_seed: Option<u64>)]
pub struct CreateRedPacket<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    // 创建者状态账户
    #[account(
        mut,
        seeds = [config::CREATOR_STATE_SEED, creator.key().as_ref()],
        bump = creator_state.bump
    )]
    pub creator_state: Account<'info, CreatorState>,

    // 红包账户
    #[account(
        init,
        payer = creator,
        space = config::RED_PACKET_SPACE,
        seeds = [
            b"red_packet", 
            creator.key().as_ref(),
            &creator_state.next_red_packet_id.to_le_bytes() // 使用固定8字节
        ],
        bump
    )]
    pub red_packet: Account<'info, RedPacket>,

    /// CHECK: This can be SOL (Pubkey::default) or SPL token mint
    #[account()]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: Creator's ATA for SPL tokens, creator for SOL
    #[account(mut)]
    pub creator_ata: UncheckedAccount<'info>,

    /// CHECK: Pool ATA for SPL tokens, red_packet account for SOL
    #[account(mut)]
    pub pool_ata: UncheckedAccount<'info>,

    #[account(
        mut,
        address = FEE_RECEIVER
    )]
    pub fee_receiver: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    #[account(
    constraint = token_program.key() == spl_token::id() || token_program.key() == spl_token_2022::id() @ RedPacketError::InvalidTokenProgram
    )]
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
