use crate::{requests::send_and_confirm_transaction, utils::get_user_keypair};
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
};
use teloxide::types::UserId;
use std::sync::Arc;

pub async fn approve_transaction(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
    user_id: UserId,
) -> Result<Signature, String> {  // Return Result to handle errors
    // Collect the instruction for approval
    let instructions: Vec<Instruction> = vec![crate::instructions::approve_transaction(
        multisig_pubkey,
        transaction_index,
        user_id,
    )];

    // Retrieve the user keypair asynchronously
    let member_keypair = match get_user_keypair(user_id).await {
        Ok(keypair) => keypair,
        Err(e) => {
            return Err(format!("Failed to get user keypair: {}", e)); // Handle keypair fetch error
        }
    };

    let member_pubkey = member_keypair.pubkey();
    let message = Message::new(&instructions, Some(&member_pubkey));

    // Prepare the signers list
    let signers: Vec<&Keypair> = vec![&member_keypair];

    // Send and confirm the transaction
    match send_and_confirm_transaction(message, signers).await {
        Ok(signature) => Ok(signature),
        Err(e) => Err(format!("Transaction failed to confirm: {}", e)), // Handle send/confirm error
    }
}
