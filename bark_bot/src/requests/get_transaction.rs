use crate::collections::Transaction;
use reqwest::{Client, Error as ReqwestError};
use std::env;

#[derive(Debug)]
pub enum TransactionError {
    MissingEnvVar(String),
    RequestError(String),
    JsonError(serde_json::Error),
}

pub async fn get_transaction(id: i64) -> Result<Transaction, TransactionError> {
    let base_url = env::var("API_BASE_URL")
        .map_err(|_| TransactionError::MissingEnvVar("API_BASE_URL".to_string()))?;

    let client = Client::new();
    let path = format!("{}/transactions/{}", base_url, id);

    let response = client
        .get(path)
        .send()
        .await
        .map_err(|e| TransactionError::RequestError(e.to_string()))?;

    // Check if the response was successful
    if response.status().is_success() {
        response
            .json::<Transaction>()
            .await
            .map_err(TransactionError::JsonError)
    } else {
        Err(TransactionError::RequestError(format!(
            "Request failed with status: {}",
            response.status()
        )))
    }
}
