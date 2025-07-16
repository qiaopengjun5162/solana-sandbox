use anchor_lang::prelude::*;

use crate::state::Initialize;

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
