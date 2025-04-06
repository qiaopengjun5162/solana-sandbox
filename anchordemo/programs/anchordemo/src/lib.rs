#![allow(unexpected_cfgs)] // 在整个 crate 中忽略

use anchor_lang::prelude::*;

declare_id!("AnkpTFgp1wzTCZHU7kxQTsit4zQZuqpY4cDzgS5bQnCc");

#[program]
pub mod anchordemo {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        number: u64,
        text: String,
        optional_key: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.data_account.number = number;
        ctx.accounts.data_account.optional_key = optional_key;
        ctx.accounts.data_account.text = text;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(input_number: u64, input_text: String)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 36 + input_text.len() + 4)]
    pub data_account: Account<'info, DemoDataAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DemoDataAccount {
    pub number: u64,
    pub optional_key: Option<Pubkey>,
    pub text: String,
}
