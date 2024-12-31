use crate::utils::RPC;
use anchor_lang::AccountDeserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use squads_mpl::state::Ms;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MultisigError {
    #[error("Failed to fetch account data")]
    FetchAccountDataError(#[from] solana_client::client_error::ClientError),
    
    #[error("Failed to deserialize multisig data")]
    DeserializeError(#[from] anchor_lang::error::Error),
}

pub async fn get_multisig_account(multisig_pubkey: Pubkey) -> Result<Ms, MultisigError> {
    let solana_client = RpcClient::new(RPC.to_string());

    // Fetch account data
    let account_data = solana_client
        .get_account_data(&multisig_pubkey)
        .await
        .map_err(MultisigError::FetchAccountDataError)?;

    // Attempt to deserialize the account data into the Ms struct
    Ms::try_deserialize(&mut &account_data[..])
        .map_err(MultisigError::DeserializeError)
}
