use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// Define the program ID for the SQUADS program
pub const SQUADS_PROGRAM_ID: Pubkey = Pubkey::from_str("BARKkeAwhTuFzcLHX4DjotRsmjXQ1MshGrZbn1CUQqMo").unwrap();

/// This function generates a program-derived address (PDA) for a transaction
/// based on the multisig public key and the transaction index.
/// 
/// # Parameters
/// - `multisig_pubkey`: The public key of the multisig account.
/// - `transaction_index`: The index of the transaction (used to create unique addresses for each transaction).
/// 
/// # Returns
/// The derived public key for the transaction associated with the provided multisig and transaction index.
pub fn get_transaction_pubkey(multisig_pubkey: Pubkey, transaction_index: u32) -> Pubkey {
    let (transaction_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",                          // Constant seed identifying the program.
            &multisig_pubkey.to_bytes(),       // The multisig public key (serialized as bytes).
            &transaction_index.to_le_bytes(),  // The transaction index (little-endian byte representation).
            b"transaction",                    // Constant seed identifying the transaction type.
        ],
        &SQUADS_PROGRAM_ID,                    // The program ID under which the PDA is derived.
    );

    transaction_pubkey
}

fn main() {
    // Example usage

    // Define the multisig public key (replace with actual value)
    let multisig_pubkey = Pubkey::from_str("BARKkeAwhTuFzcLHX4DjotRsmjXQ1MshGrZbn1CUQqMo").unwrap();
    
    // Define the transaction index (example: first transaction)
    let transaction_index = 1;

    // Get the program-derived address (PDA) for the transaction
    let transaction_pubkey = get_transaction_pubkey(multisig_pubkey, transaction_index);

    // Print the resulting transaction public key
    println!("Transaction Pubkey: {}", transaction_pubkey);
}
