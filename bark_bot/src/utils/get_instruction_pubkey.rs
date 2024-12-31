use crate::utils::SQUADS_PROGRAM_ID;
use solana_sdk::pubkey::Pubkey;

/// Derives a unique program address for the given instruction in the specified transaction.
///
/// # Arguments:
/// * `transaction_pubkey` - The public key of the transaction that this instruction belongs to.
/// * `instruction_index` - The index of the instruction within the transaction.
///
/// # Returns:
/// * The derived `Pubkey` for the instruction.
pub fn get_instruction_pubkey(transaction_pubkey: Pubkey, instruction_index: u8) -> Pubkey {
    // Derive the program address using the transaction pubkey and instruction index as seeds
    let (instruction_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",                           // Constant seed for program
            &transaction_pubkey.to_bytes(),     // Transaction's pubkey as a seed
            &instruction_index.to_le_bytes(),  // Instruction index (little-endian)
            b"instruction",                     // Constant seed for instruction
        ],
        &SQUADS_PROGRAM_ID,                    // The Solana program ID for the Squads program
    );

    instruction_pubkey
}
