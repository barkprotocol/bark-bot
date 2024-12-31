use crate::{requests::send_and_confirm_transaction, utils::get_user_keypair};
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
};
use teloxide::types::UserId;

pub async fn execute_transaction(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
    user_id: UserId,
) -> Result<Signature, String> {  // Return Result to handle errors
    // Construct the instructions
    let instructions: Vec<Instruction> = vec![crate::instructions::execute_transaction(
        multisig_pubkey,
        transaction_index,
        user_id,
    )];

    // Get the user keypair
    let member_keypair = match get_user_keypair(user_id).await {
        Ok(keypair) => keypair,
        Err(e) => return Err(format!("Failed to get user keypair: {}", e)),
    };

    let member_pubkey = member_keypair.pubkey();
    let message = Message::new(&instructions, Some(&member_pubkey));

    // Prepare the list of signers
    let signers: Vec<&Keypair> = vec![&member_keypair];

    // Send the transaction and confirm it
    match send_and_confirm_transaction(message, signers).await {
        Ok(signature) => Ok(signature),  // Return signature if successful
        Err(e) => Err(format!("Failed to send and confirm transaction: {}", e)),  // Return error if failed
    }
}
