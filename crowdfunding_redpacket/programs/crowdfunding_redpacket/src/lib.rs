#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use state::*;

declare_id!("3jSB715HJHpXnJNeoABw6nAzg9hJ4bgGERumnsoAa31X");

#[program]
pub mod crowdfunding_redpacket {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn create_custom_redpacket(
        ctx: Context<CreateCustomRedpacket>,
        params: CustomCrowdfundingParams,
    ) -> Result<()> {
        instructions::create::handler(ctx, params)
    }

    pub fn claim_airdrop(ctx: Context<ClaimAirdrop>) -> Result<()> {
        instructions::airdrop::handler(ctx)
    }

    pub fn support_crowdfunding(ctx: Context<SupportCrowdfunding>, amount: u64) -> Result<()> {
        instructions::support::handler(ctx, amount)
    }

    pub fn settle_crowdfunding(ctx: Context<SettleCrowdfunding>) -> Result<()> {
        instructions::settle::handler(ctx)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::refund::handler(ctx)
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
        instructions::claim_tokens::handler(ctx)
    }

    pub fn claim_dev_fund(ctx: Context<ClaimDevFund>) -> Result<()> {
        instructions::claim_dev_fund::handler(ctx)
    }

    pub fn distribute_fees(ctx: Context<DistributeFees>) -> Result<()> {
        instructions::distribute_fees::handler(ctx)
    }

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        developer_wallet: Pubkey,
    ) -> Result<()> {
        instructions::initialize_config::handler(ctx, developer_wallet)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, new_developer_wallet: Pubkey) -> Result<()> {
        instructions::update_config::handler(ctx, new_developer_wallet)
    }
}
