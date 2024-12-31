use crate::{requests::send_and_confirm_transaction, utils::get_user_keypair};
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
};
use teloxide::types::UserId;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RejectTransactionError {
    #[error("Failed to get user keypair")]
    GetUserKeypairError,

    #[error("Failed to send and confirm transaction")]
    TransactionSendError,
}

pub async fn reject_transaction(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
    user_id: UserId,
) -> Result<Signature, RejectTransactionError> {
    // Prepare instructions
    let instructions: Vec<Instruction> = vec![crate::instructions::reject_transaction(
        multisig_pubkey,
        transaction_index,
        user_id,
    )];

    // Get user keypair
    let member_keypair = get_user_keypair(user_id).map_err(|_| RejectTransactionError::GetUserKeypairError)?;
    let member_pubkey = member_keypair.pubkey();

    // Create the message
    let message = Message::new(&instructions, Some(&member_pubkey));
    let signers: Vec<&Keypair> = vec![&member_keypair];

    // Send and confirm the transaction
    send_and_confirm_transaction(message, signers)
        .await
        .map_err(|_| RejectTransactionError::TransactionSendError)
}
