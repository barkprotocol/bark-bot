use crate::utils::{
    get_instruction_pubkey, get_program, get_transaction_pubkey, get_user_keypair,
    SQUADS_PROGRAM_ID,
};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer, system_program};
use squads_mpl::state::{IncomingInstruction, MsAccountMeta};
use teloxide::types::UserId;

pub fn add_instruction(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
    instruction_index: u8,
    instruction: Instruction,
    user_id: UserId,
) -> Result<Instruction, String> {
    // Retrieve the user's keypair
    let creator_keypair = get_user_keypair(user_id);
    let creator_pubkey = creator_keypair.pubkey();

    // Get the program instance
    let program = get_program(creator_keypair, SQUADS_PROGRAM_ID);

    // Get the transaction public key and instruction public key
    let transaction_pubkey = get_transaction_pubkey(multisig_pubkey, transaction_index);
    let instruction_pubkey = get_instruction_pubkey(transaction_pubkey, instruction_index);

    // Attempt to create the instruction
    program
        .request()
        .accounts(squads_mpl::accounts::AddInstruction {
            system_program: system_program::ID,
            multisig: multisig_pubkey,
            transaction: transaction_pubkey,
            creator: creator_pubkey,
            instruction: instruction_pubkey,
        })
        .args(squads_mpl::instruction::AddInstruction {
            incoming_instruction: IncomingInstruction {
                data: instruction.data,
                program_id: instruction.program_id,
                keys: instruction
                    .accounts
                    .iter()
                    .map(|account| MsAccountMeta {
                        is_signer: account.is_signer,
                        is_writable: account.is_writable,
                        pubkey: account.pubkey,
                    })
                    .collect(),
            },
        })
        .instructions()
        .and_then(|instructions| instructions.first().cloned()) // Safely get the first instruction
        .ok_or_else(|| "Failed to generate instruction".to_string()) // Return an error if no instructions are found
}
