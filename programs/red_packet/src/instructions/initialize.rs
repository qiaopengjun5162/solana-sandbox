use crate::{
    config::{CREATOR_STATE_SEED, CREATOR_STATE_SPACE},
    states::CreatorState,
};
use anchor_lang::prelude::*;

pub fn handler_creator_state(ctx: Context<InitializeCreatorState>) -> Result<()> {
    ctx.accounts.creator_state.next_red_packet_id = 0;
    ctx.accounts.creator_state.bump = ctx.bumps.creator_state;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCreatorState<'info> {
    #[account(
        init,
        payer = creator,
        space = CREATOR_STATE_SPACE,
        seeds = [CREATOR_STATE_SEED, creator.key().as_ref()],
        bump
    )]
    pub creator_state: Account<'info, CreatorState>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
