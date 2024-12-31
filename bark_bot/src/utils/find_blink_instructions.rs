use anchor_lang::prelude::AccountMeta;
use base64::prelude::*;
use solana_sdk::{instruction::Instruction, transaction::Transaction};
use std::error::Error;

#[derive(Debug)]
pub enum BlinkInstructionError {
    Base64DecodeError(base64::DecodeError),
    BincodeDeserializeError(bincode::Error),
    InstructionError(String),
}

pub fn find_blink_instructions(transaction: String) -> Result<Vec<Instruction>, BlinkInstructionError> {
    // Decode the base64 string into bytes
    let blink_transaction_as_bytes = BASE64_STANDARD
        .decode(&transaction)
        .map_err(BlinkInstructionError::Base64DecodeError)?;

    // Deserialize the transaction into a Solana Transaction
    let blink_transaction: Transaction = bincode::deserialize(&blink_transaction_as_bytes)
        .map_err(BlinkInstructionError::BincodeDeserializeError)?;

    // Map the instructions into Solana Instruction objects
    blink_transaction
        .message
        .instructions
        .iter()
        .map(|instruction| {
            let program_id = blink_transaction.message.account_keys
                .get(instruction.program_id_index as usize)
                .ok_or_else(|| BlinkInstructionError::InstructionError("Invalid program_id_index".to_string()))?;

            let accounts: Result<Vec<AccountMeta>, BlinkInstructionError> = instruction
                .accounts
                .iter()
                .map(|account_index| {
                    let pubkey = blink_transaction.message.account_keys
                        .get(*account_index as usize)
                        .ok_or_else(|| BlinkInstructionError::InstructionError("Invalid account index".to_string()))?;

                    let is_signer = blink_transaction.message.is_signer(*account_index as usize);
                    let is_writable = blink_transaction.message.is_writable(*account_index as usize);

                    if is_writable {
                        Ok(AccountMeta::new(*pubkey, is_signer))
                    } else {
                        Ok(AccountMeta::new_readonly(*pubkey, is_signer))
                    }
                })
                .collect();

            // Check if the accounts map was successful
            accounts.map(|accounts| Instruction {
                program_id: *program_id,
                data: instruction.data.clone(),
                accounts,
            })
        })
        .collect()
}

