use solana_sdk::signature::Keypair;
use std::env;
use teloxide::types::UserId;
use serde_json;

pub fn get_user_keypair(user_id: UserId) -> Keypair {
    // Step 1: Retrieve the whitelist from the environment variable
    let whitelist = env::var("WHITELIST").unwrap();

    // Step 2: Parse the whitelist JSON string into a vector of tuples (user_id, private_key)
    let parsed_whitelist: Vec<(String, String)> = serde_json::from_str(&whitelist).unwrap();

    // Step 3: Find the private key for the user ID
    let private_key = parsed_whitelist
        .iter()
        .find(|entry| entry.0 == user_id.to_string()) // Find entry for the user_id
        .unwrap_or_else(|| panic!("User {} not found in whitelist!", user_id)) // Panic if not found
        .1
        .clone(); // Get the private key as a string

    // Step 4: Create and return the Keypair from the private key string (base58 format)
    Keypair::from_base58_string(&private_key)
}

fn main() {
    // Example usage of the function
    let user_id = UserId(12345); // Example UserId
    let keypair = get_user_keypair(user_id);
    
    println!("User Keypair: {:?}", keypair);
}
