use crate::{config, events::RedPacketRefunded, RedPacket, RedPacketError};
use anchor_lang::prelude::*;
use anchor_spl::token::spl_token;
use anchor_spl::token_interface::{self, TokenInterface, TransferChecked};
use spl_token_2022::extension::StateWithExtensions;
use spl_token_2022::state::Mint as Mint2022;

pub fn refund_handler(ctx: Context<Refund>, red_packet_id: u64) -> Result<()> {
    let red_packet = &mut ctx.accounts.red_packet;
    let red_packet_key = red_packet.key();

    require!(
        red_packet.red_packet_id == red_packet_id,
        RedPacketError::InvalidRedPacketId
    );
    require!(
        Clock::get()?.unix_timestamp >= red_packet.expiry_time,
        RedPacketError::RedPacketNotExpired
    );

    require!(
        ctx.accounts.creator.key() == red_packet.creator,
        RedPacketError::Unauthorized
    );

    require!(
        red_packet.remaining_amount > 0,
        RedPacketError::NoFundsToRefund
    );

    let refund_amount = red_packet.remaining_amount;

    if red_packet.is_sol {
        let rent_exempt_lamports = Rent::get()?.minimum_balance(config::RED_PACKET_SPACE);
        let red_packet_lamports = red_packet.to_account_info().lamports();
        require!(
            red_packet_lamports >= refund_amount + rent_exempt_lamports,
            RedPacketError::NoFundsToRefund
        );

        **red_packet.to_account_info().try_borrow_mut_lamports()? -= refund_amount;
        **ctx
            .accounts
            .creator
            .to_account_info()
            .try_borrow_mut_lamports()? += refund_amount;
    } else {
        let mint_data = ctx.accounts.mint.data.borrow();
        let mint_account = StateWithExtensions::<Mint2022>::unpack(&mint_data)?;
        let decimals = mint_account.base.decimals;
        token_interface::transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.pool_ata.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.creator_ata.to_account_info(),
                    authority: red_packet.to_account_info(),
                },
                &[&[
                    b"red_packet",
                    red_packet.creator.as_ref(),
                    &red_packet.red_packet_id.to_le_bytes(),
                    &[red_packet.bump],
                ]],
            ),
            refund_amount,
            decimals,
        )?;

        token_interface::close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_interface::CloseAccount {
                account: ctx.accounts.pool_ata.to_account_info(),
                destination: ctx.accounts.creator.to_account_info(),
                authority: red_packet.to_account_info(),
            },
            &[&[
                b"red_packet",
                red_packet.creator.as_ref(),
                &red_packet.red_packet_id.to_le_bytes(),
                &[red_packet.bump],
            ]],
        ))?;
    }

    red_packet.remaining_amount = 0;

    // 记录事件 - 在关闭账户前
    emit!(RedPacketRefunded {
        creator: ctx.accounts.creator.key(),
        red_packet: red_packet_key,
        amount: refund_amount,
        red_packet_id
    });

    // 关闭红包账户
    let red_packet_account_info = ctx.accounts.red_packet.to_account_info();
    let dest = ctx.accounts.creator.to_account_info();
    // 转移剩余lamports
    let lamports = red_packet_account_info.lamports();
    if lamports > 0 {
        **red_packet_account_info.try_borrow_mut_lamports()? -= lamports;
        **dest.try_borrow_mut_lamports()? += lamports;
    }
    // 清空账户数据
    let mut data = red_packet_account_info.try_borrow_mut_data()?;
    data.fill(0);

    Ok(())
}

#[derive(Accounts)]
#[instruction(red_packet_id: u64)]
pub struct Refund<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
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
    constraint = token_program.key() == spl_token::id() || token_program.key() == spl_token_2022::id() @ RedPacketError::InvalidTokenProgram
    )]
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
