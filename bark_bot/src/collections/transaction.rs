#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub id: i64,                          // Unique identifier for the transaction
    pub transaction_index: u32,            // Index of the transaction in a sequence (e.g., for a multisig transaction)
    pub user_id: String,                   // The user ID associated with this transaction
    pub signature: String,                 // The signature of the transaction (usually used to verify authenticity)
    pub status: u8,                        // The status of the transaction (possibly as an integer, e.g., 0 for pending, 1 for completed)
    pub message_id: Option<String>,        // Optional field for a message ID related to the transaction, could be null
}
