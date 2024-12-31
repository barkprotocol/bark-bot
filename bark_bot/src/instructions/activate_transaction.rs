use crate::utils::{get_program, get_transaction_pubkey, get_user_keypair, SQUADS_PROGRAM_ID};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer};
use teloxide::types::UserId;

pub fn activate_transaction(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
    user_id: UserId,
) -> Result<Instruction, String> {
    // Retrieve the user's keypair from the provided user_id
    let creator_keypair = get_user_keypair(user_id);
    let creator_pubkey = creator_keypair.pubkey();

    // Initialize the program instance with the user's keypair and the program ID
    let program = get_program(creator_keypair, SQUADS_PROGRAM_ID);

    // Get the public key of the specific transaction
    let transaction_pubkey = get_transaction_pubkey(multisig_pubkey, transaction_index);

    // Attempt to build the instruction
    program
        .request()
        .accounts(squads_mpl::accounts::ActivateTransaction {
            multisig: multisig_pubkey,
            transaction: transaction_pubkey,
            creator: creator_pubkey,
        })
        .args(squads_mpl::instruction::ActivateTransaction) // Ensure this matches the correct argument for activation
        .instructions()
        .and_then(|instructions| instructions.first().cloned()) // Safely unwrap the first instruction
        .ok_or_else(|| "Failed to generate instruction".to_string()) // Return an error if no instruction is found
}
