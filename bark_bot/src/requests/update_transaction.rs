use crate::collections::Transaction;
use serde::{Deserialize, Serialize};
use std::env;
use teloxide::types::MessageId;
use reqwest::{Client, Error};

#[derive(Serialize, Deserialize)]
pub struct UpdateTransactionBody {
    pub message_id: String,
}

#[derive(Debug)]
pub enum UpdateTransactionError {
    RequestError(Error),
    ApiError(String),
}

pub async fn update_transaction(id: i64, message_id: MessageId) -> Result<Transaction, UpdateTransactionError> {
    let base_url = env::var("API_BASE_URL").map_err(|e| UpdateTransactionError::ApiError(e.to_string()))?;
    let body = UpdateTransactionBody {
        message_id: message_id.to_string(),
    };
    
    let client = Client::new();
    let path = format!("{}/transactions/{}", base_url, id);

    let response = client
        .patch(path)
        .json(&body)
        .send()
        .await
        .map_err(UpdateTransactionError::RequestError)?;

    if response.status().is_success() {
        response
            .json::<Transaction>()
            .await
            .map_err(|e| UpdateTransactionError::RequestError(e))
    } else {
        Err(UpdateTransactionError::ApiError(format!(
            "Failed to update transaction. Status: {}",
            response.status()
        )))
    }
}
