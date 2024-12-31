use crate::utils::RPC;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    message::Message,
    signature::{Keypair, Signature},
    transaction::Transaction,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Failed to get the latest blockhash")]
    BlockhashError(#[from] solana_client::rpc_request::RpcError),
    
    #[error("Failed to send and confirm the transaction")]
    SendError(#[from] solana_client::rpc_request::RpcError),
}

pub async fn send_and_confirm_transaction(
    message: Message,
    signers: Vec<&Keypair>,
) -> Result<Signature, TransactionError> {
    let solana_client = RpcClient::new(RPC.to_string());
    
    // Get the latest blockhash
    let blockhash = solana_client
        .get_latest_blockhash()
        .await
        .map_err(TransactionError::BlockhashError)?;
    
    // Create the transaction
    let transaction = Transaction::new(&signers, message, blockhash);
    
    // Send and confirm the transaction
    let signature = solana_client
        .send_and_confirm_transaction(&transaction)
        .await
        .map_err(TransactionError::SendError)?;
    
    Ok(signature)
}
