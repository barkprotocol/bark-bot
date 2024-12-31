use anchor_client::{Client, Cluster, Program};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::{rc::Rc, str::FromStr};

/// Returns an Anchor `Program` for interacting with a specific program on Solana.
///
/// # Parameters:
/// - `signer`: A `Keypair` used to sign transactions.
/// - `program_id`: A `Pubkey` representing the program ID to interact with.
///
/// # Returns:
/// - `Ok(Program)` if the client is successfully created and connected.
/// - `Err(String)` if there is an error creating the client or connecting to the program.
pub fn get_program(signer: Keypair, program_id: Pubkey) -> Result<Program, String> {
    // Fetch the RPC URL from the environment or configuration
    let rpc_url = crate::utils::RPC;

    // Create a Cluster from the RPC URL
    let cluster = Cluster::from_str(rpc_url)
        .map_err(|_| format!("Failed to parse RPC URL: {}", rpc_url))?;

    // Create the Anchor client with the signer and the cluster
    let client = Client::new(cluster, Rc::new(signer));

    // Return the program client
    Ok(client.program(program_id))
}
