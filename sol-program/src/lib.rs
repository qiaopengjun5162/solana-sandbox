// use solana_program::entrypoint;
// use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use solana_account_info::AccountInfo;
use solana_program_entrypoint::entrypoint;
use solana_program_error::ProgramResult;
use solana_pubkey::Pubkey;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // msg!("Hello, world!");
    Ok(())
}


