use crate::utils::{get_program, get_transaction_pubkey, get_user_keypair, SQUADS_PROGRAM_ID};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer};
use teloxide::types::UserId;

pub fn approve_transaction(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
    user_id: UserId,
) -> Result<Instruction, String> {
    // Retrieve the member's keypair
    let member_keypair = get_user_keypair(user_id);
    let member_pubkey = member_keypair.pubkey();

    // Get the program instance
    let program = get_program(member_keypair, SQUADS_PROGRAM_ID);

    // Get the transaction public key
    let transaction_pubkey = get_transaction_pubkey(multisig_pubkey, transaction_index);

    // Attempt to create the instruction
    program
        .request()
        .accounts(squads_mpl::accounts::VoteTransaction {
            multisig: multisig_pubkey,
            transaction: transaction_pubkey,
            member: member_pubkey,
        })
        .args(squads_mpl::instruction::ApproveTransaction)
        .instructions()
        .and_then(|instructions| instructions.first().cloned()) // Safely get the first instruction
        .ok_or_else(|| "Failed to generate instruction".to_string()) // Return an error if no instructions are found
}
