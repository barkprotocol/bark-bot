use crate::utils::SQUADS_PROGRAM_ID;
use solana_sdk::pubkey::Pubkey;

/// Derives the public key for a multisig authority in the specified multisig account.
///
/// # Arguments:
/// * `multisig_pubkey` - The public key of the multisig account.
/// * `authority` - The index of the multisig authority.
///
/// # Returns:
/// * The derived `Pubkey` for the multisig authority.
pub fn get_multisig_authority_pubkey(multisig_pubkey: Pubkey, authority: u32) -> Pubkey {
    // Derive the multisig authority's program address using the multisig pubkey and authority index as seeds
    let (multisig_authority_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",                              // Constant seed for the program
            &multisig_pubkey.to_bytes(),           // Multisig account's pubkey as a seed
            &authority.to_le_bytes(),             // Authority index (little-endian)
            b"authority",                          // Constant seed for authority
        ],
        &SQUADS_PROGRAM_ID,                       // The Solana program ID for the Squads program
    );

    multisig_authority_pubkey
}
