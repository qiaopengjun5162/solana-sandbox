#![allow(unexpected_cfgs)]

use borsh_derive::{
    BorshDeserialize as BorshDeserializeDerive, BorshSerialize as BorshSerializeDerive,
};
use solana_account_info::AccountInfo;
use solana_msg::msg;
use solana_program_entrypoint::entrypoint;
use solana_program_error::{ProgramError, ProgramResult};
use solana_pubkey::Pubkey; // 为 trait 设置别名

// 定义事件结构体
#[derive(BorshDeserializeDerive, BorshSerializeDerive, Debug)]
pub struct GreetingEvent {
    pub message: String, // Greeting message contained in the event
}

// 自定义事件触发函数
fn emit_event(event: &GreetingEvent) -> ProgramResult {
    let event_data = borsh::to_vec(event).map_err(|_| ProgramError::Custom(1))?; // Serialize to byte array
    msg!("EVENT:GREETING:{:?}", event_data); // Output event log
    Ok(())
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, Solana!");

    let event = GreetingEvent {
        message: "Hello from Solana program!".to_string(),
    };
    emit_event(&event)?;

    msg!("Program executed successfully with greeting event!");

    Ok(())
}

#[cfg(test)]
mod test {
    use solana_program_test::*;
    use solana_sdk::{
        instruction::Instruction, pubkey::Pubkey, signature::Signer, transaction::Transaction,
    };
    use std::str::FromStr;

    #[tokio::test]
    async fn test_sol_program() {
        let program_id = Pubkey::from_str("GGBjDqYdicSE6Qmtu6SAsueX1biM5LjbJ8R8vZvFfofA").unwrap();
        let mut program_test = ProgramTest::default();
        program_test.add_program("sol_program", program_id, None);
        let mut context = program_test.start_with_context().await;
        let (banks_client, payer, recent_blockhash) = (
            &mut context.banks_client,
            &context.payer,
            context.last_blockhash,
        );
        // Create instruction
        let instruction = Instruction {
            program_id,
            accounts: vec![],
            data: vec![],
        };
        // Create transaction with instruction
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));

        // Sign transaction
        transaction.sign(&[&payer], recent_blockhash);

        let transaction_result = banks_client
            .process_transaction_with_metadata(transaction)
            .await
            .unwrap();

        assert!(transaction_result.result.is_ok());

        let logs = transaction_result.metadata.unwrap().log_messages;
        assert!(logs.iter().any(|log| log.contains("Hello, Solana!")));
        assert!(logs.iter().any(|log| log.contains("EVENT:GREETING:")));
    }
}
