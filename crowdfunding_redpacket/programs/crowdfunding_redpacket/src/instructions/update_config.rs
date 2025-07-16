// in instructions/update_config.rs

use crate::state::UpdateConfig;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<UpdateConfig>, new_developer_wallet: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.developer_wallet = new_developer_wallet;
    // msg!("Developer wallet updated to: {}", new_developer_wallet);
    Ok(())
}
