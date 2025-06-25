#![allow(unexpected_cfgs)]

use crate::{
    config::{self, CLAIM_FEE, FEE_RECEIVER},
    events::RedPacketClaimed,
    utils, RedPacket, RedPacketError, UserState,
};
use anchor_lang::{
    prelude::*,
    solana_program::{program_pack::Pack, pubkey::Pubkey},
    system_program,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::spl_token,
    token_interface::{self, TokenInterface, TransferChecked},
};
use spl_token::state::Mint as MintLegacy;
use spl_token_2022::{extension::StateWithExtensions, state::Mint as Mint2022}; // 引入 SPL Token 的 Mint 结构

pub fn claim_handler(
    ctx: Context<ClaimRedPacket>,
    amount: Option<u64>, // 仅 Merkle 树红包需要提供 amount
    proof: Option<Vec<[u8; 32]>>,
    red_packet_id: u64,
) -> Result<()> {
    let red_packet = &mut ctx.accounts.red_packet;
    let user_state = &mut ctx.accounts.user_state;
    let red_packet_key = red_packet.key();
    let clock = Clock::get()?;
    require!(user_state.is_claimed == 0, RedPacketError::AlreadyClaimed);
    require!(
        red_packet.red_packet_id == red_packet_id,
        RedPacketError::InvalidRedPacketId
    );
    require!(
        clock.unix_timestamp < red_packet.expiry_time,
        RedPacketError::RedPacketExpired
    );

    require!(
        red_packet.claimed_count < red_packet.packet_count,
        RedPacketError::NoPacketsRemaining
    );

    let claim_amount = match red_packet.red_packet_type {
        0 => red_packet.share_amount,
        1 => utils::calculate_random_amount(red_packet, ctx.accounts.claimer.key())?,
        2 => {
            let claim_amount = amount.ok_or(RedPacketError::InvalidClaimAmount)?;
            let proof_vec = proof.ok_or(RedPacketError::MerkleProofInvalid)?;
            utils::verify_merkle_proof(
                ctx.accounts.claimer.key(),
                claim_amount,
                &proof_vec,
                &red_packet.merkle_root,
            )?;
            claim_amount
        }
        _ => return Err(RedPacketError::InvalidRedPacketType.into()),
    };

    require!(claim_amount > 0, RedPacketError::InvalidClaimAmount);
    require!(
        red_packet.remaining_amount >= claim_amount,
        RedPacketError::InsufficientFunds
    );

    if red_packet.is_sol {
        let red_packet_lamports = red_packet.to_account_info().lamports();
        require!(
            red_packet_lamports >= claim_amount + CLAIM_FEE,
            RedPacketError::InsufficientFunds
        );
        require!(
            ctx.accounts.claimer.lamports() >= CLAIM_FEE,
            RedPacketError::InsufficientClaimerFunds
        );
        **red_packet.to_account_info().try_borrow_mut_lamports()? -= claim_amount;
        **ctx.accounts.user_ata.try_borrow_mut_lamports()? += claim_amount;

        **ctx.accounts.claimer.try_borrow_mut_lamports()? -= CLAIM_FEE;
        **ctx
            .accounts
            .fee_receiver
            .to_account_info()
            .try_borrow_mut_lamports()? += CLAIM_FEE;
    } else {
        let mint_data = ctx.accounts.mint.data.borrow();
        let decimals = if ctx.accounts.token_program.key() == spl_token::id() {
            // 处理 SPL Token
            let mint =
                MintLegacy::unpack(&mint_data).map_err(|_| RedPacketError::InvalidMintAccount)?;
            mint.decimals
        } else if ctx.accounts.token_program.key() == spl_token_2022::id() {
            // 处理 Token-2022
            let mint = StateWithExtensions::<Mint2022>::unpack(&mint_data)
                .map_err(|_| RedPacketError::InvalidMintAccount)?;
            // 暂时移除铸币权限检查，后续在第二步添加扩展检查
            mint.base.decimals
        } else {
            return Err(RedPacketError::InvalidTokenProgram.into());
        };

        if ctx.accounts.user_ata.data_is_empty() {
            anchor_spl::associated_token::create(CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: ctx.accounts.claimer.to_account_info(),
                    associated_token: ctx.accounts.user_ata.to_account_info(),
                    authority: ctx.accounts.claimer.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
            ))?;
        }

        token_interface::transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.pool_ata.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.user_ata.to_account_info(),
                    authority: red_packet.to_account_info(),
                },
                &[&[
                    b"red_packet",
                    red_packet.creator.as_ref(),
                    &red_packet.red_packet_id.to_le_bytes(),
                    &[red_packet.bump],
                ]],
            ),
            claim_amount,
            decimals,
        )?;

        require!(
            ctx.accounts.claimer.lamports() >= CLAIM_FEE,
            RedPacketError::InsufficientClaimerFunds
        );
        //  CLAIM_FEE 转移（从 claimer 到 fee_vault）
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.claimer.to_account_info(),
                    to: ctx.accounts.fee_receiver.to_account_info(),
                },
            ),
            CLAIM_FEE,
        )?;
    }

    user_state.is_claimed = 1;
    red_packet.remaining_amount -= claim_amount;
    red_packet.claimed_count += 1;

    emit!(RedPacketClaimed {
        claimer: ctx.accounts.claimer.key(),
        red_packet: red_packet_key,
        amount: claim_amount,
        red_packet_id
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(red_packet_id: u64)]
pub struct ClaimRedPacket<'info> {
    #[account(mut)]
    pub claimer: Signer<'info>,

    #[account(mut)]
    pub red_packet: Account<'info, RedPacket>,

    #[account(
        init_if_needed,
        payer = claimer,
        space = config::USER_STATE_SPACE,
        seeds = [b"user_state", red_packet.key().as_ref(), claimer.key().as_ref()],
        bump
    )]
    pub user_state: Account<'info, UserState>,

    /// CHECK: This can be SOL (Pubkey::default) or SPL token mint
    #[account()]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: Pool ATA for SPL tokens, red_packet account for SOL
    #[account(mut)]
    pub pool_ata: UncheckedAccount<'info>,

    /// CHECK: User's ATA for SPL tokens, user for SOL
    #[account(mut)]
    pub user_ata: UncheckedAccount<'info>,

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
}
