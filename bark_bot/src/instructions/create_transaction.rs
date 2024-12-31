use crate::utils::{get_program, get_transaction_pubkey, get_user_keypair, SQUADS_PROGRAM_ID};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer, system_program};
use teloxide::types::UserId;

pub fn create_transaction(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
    user_id: UserId,
) -> Result<Instruction, String> {
    // Retrieve the creator's keypair
    let creator_keypair = get_user_keypair(user_id);
    let creator_pubkey = creator_keypair.pubkey();

    // Get the program instance
    let program = get_program(creator_keypair, SQUADS_PROGRAM_ID);

    // Get the transaction public key
    let transaction_pubkey = get_transaction_pubkey(multisig_pubkey, transaction_index);

    // Attempt to create the instruction
    program
        .request()
        .accounts(squads_mpl::accounts::CreateTransaction {
            system_program: system_program::ID,
            multisig: multisig_pubkey,
            transaction: transaction_pubkey,
            creator: creator_pubkey,
        })
        .args(squads_mpl::instruction::CreateTransaction { authority_index: 1 })
        .instructions()
        .and_then(|instructions| instructions.first().cloned()) // Safely get the first instruction
        .ok_or_else(|| "Failed to generate instruction".to_string()) // Return an error if no instructions are found
}
