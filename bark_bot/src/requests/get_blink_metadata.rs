use crate::collections::BlinkMetadata;
use reqwest::{Client, Error};

pub async fn get_blink_metadata(url: &str) -> Result<BlinkMetadata, Error> {
    let client = Client::new();

    // Send GET request and await the response
    let blink_response = client.get(url).send().await.map_err(|e| {
        println!("Request failed: {}", e); // You can use a logging crate here if needed
        e
    })?;

    // Parse the response as JSON into BlinkMetadata
    blink_response
        .json::<BlinkMetadata>()
        .await
        .map_err(|e| {
            println!("Failed to parse metadata: {}", e);
            e
        })
}
