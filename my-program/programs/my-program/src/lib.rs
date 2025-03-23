use anchor_lang::prelude::*;

declare_id!("Gu2NvqCH2c5kShV3XtKrE7oxHNj9k5hJmzYCXSWVoE7E");

#[program]
pub mod my_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
