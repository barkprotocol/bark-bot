use crate::collections::Transaction;
use serde::{Deserialize, Serialize};
use std::env;
use teloxide::types::UserId;
use reqwest::Error;

#[derive(Serialize, Deserialize)]
pub struct CreateTransactionBody {
    pub transaction_index: i32,
    pub user_id: String,
    pub signature: String,
}

pub async fn create_transaction(
    transaction_index: i32,
    user_id: UserId,
    signature: String,
) -> Result<Transaction, String> {
    // Fetch API base URL from environment variables
    let base_url = env::var("API_BASE_URL").map_err(|_| "API_BASE_URL environment variable is not set".to_string())?;
    
    // Prepare the body for the POST request
    let body = CreateTransactionBody {
        transaction_index,
        user_id: user_id.to_string(),
        signature,
    };
    
    // Create the HTTP client
    let client = reqwest::Client::new();
    let path = format!("{}/transactions", base_url);

    // Send the POST request and handle errors
    let response = client
        .post(&path)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    // Check if the request was successful
    if response.status().is_success() {
        // Parse the JSON response to a Transaction struct
        response.json::<Transaction>()
            .await
            .map_err(|e| format!("Failed to parse response JSON: {}", e))
    } else {
        Err(format!("Request failed with status: {}", response.status()))
    }
}
