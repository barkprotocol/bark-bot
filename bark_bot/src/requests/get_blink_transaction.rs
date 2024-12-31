use crate::utils::get_multisig_authority_pubkey;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBlinkTransactionResponse {
    pub transaction: String,
    pub message: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlinkTransactionBody {
    pub account: String,
}

pub async fn get_blink_transaction(
    multisig_pubkey: Pubkey,
    url: &str, // Changed to &str for more flexibility
) -> Result<GetBlinkTransactionResponse, Error> {
    let client = Client::new();
    let multisig_authority_pubkey = get_multisig_authority_pubkey(multisig_pubkey, 1);
    let body = BlinkTransactionBody {
        account: multisig_authority_pubkey.to_string(),
    };

    // Send POST request and await response
    let blink_response = client
        .post(url)
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            println!("POST request failed: {}", e);
            e
        })?;

    // Parse the response directly
    blink_response
        .json::<GetBlinkTransactionResponse>()
        .await
        .map_err(|e| {
            println!("Transaction failed: {}", e);
            e
        })
        .map(|response| {
            println!("ACTION TRANSACTION: \n\n{:?}", response);
            response
        })
}
