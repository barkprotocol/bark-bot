use solana_sdk::pubkey::Pubkey;
use std::{env, str::FromStr};

/// Fetches the multisig public key from the environment variable `MULTISIG_PUBKEY`
/// and converts it to a `Pubkey`.
///
/// # Returns
/// - `Ok(Pubkey)` if the environment variable is found and parsed successfully.
/// - `Err(String)` if the environment variable is not set or parsing fails.
pub fn get_multisig_pubkey() -> Result<Pubkey, String> {
    // Fetch the value of the MULTISIG_PUBKEY environment variable
    let multisig_pubkey = env::var("MULTISIG_PUBKEY")
        .map_err(|_| "MULTISIG_PUBKEY environment variable not set")?;

    // Try to parse the string into a Pubkey
    Pubkey::from_str(&multisig_pubkey)
        .map_err(|_| "Failed to parse MULTISIG_PUBKEY into a valid Pubkey")
}
