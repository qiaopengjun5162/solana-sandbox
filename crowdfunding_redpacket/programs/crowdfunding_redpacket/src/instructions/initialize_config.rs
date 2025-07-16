// in instructions/initialize_config.rs

use crate::state::InitializeConfig;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<InitializeConfig>, developer_wallet: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.admin = ctx.accounts.admin.key();
    config.developer_wallet = developer_wallet;
    // msg!("Config account initialized!");
    // msg!("Admin: {}", config.admin);
    // msg!("Developer Wallet: {}", config.developer_wallet);
    Ok(())
}
