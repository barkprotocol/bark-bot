use crate::utils::{RPC, SQUADS_PROGRAM_ID};
use anchor_lang::AccountDeserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use squads_mpl::state::MsTransaction;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Failed to fetch transaction account data")]
    FetchAccountDataError(#[from] solana_client::client_error::ClientError),
    
    #[error("Failed to deserialize transaction data")]
    DeserializeError(#[from] anchor_lang::error::Error),
    
    #[error("Transaction account not found")]
    AccountNotFound,
}

pub async fn get_transaction_account(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
) -> Result<MsTransaction, TransactionError> {
    let solana_client = RpcClient::new(RPC.to_string());

    // Find the transaction public key for the given multisig and transaction index
    let (transaction_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",
            &multisig_pubkey.to_bytes(),
            &transaction_index.to_le_bytes(),
            b"transaction",
        ],
        &SQUADS_PROGRAM_ID,
    );

    // Fetch account data for the transaction
    let transaction_data = solana_client
        .get_account_data(&transaction_pubkey)
        .await
        .map_err(TransactionError::FetchAccountDataError)?;

    // If no data was found, return an AccountNotFound error
    if transaction_data.is_empty() {
        return Err(TransactionError::AccountNotFound);
    }

    // Attempt to deserialize the transaction data
    MsTransaction::try_deserialize(&mut &transaction_data[..])
        .map_err(TransactionError::DeserializeError)
}
