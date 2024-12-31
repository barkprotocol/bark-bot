use crate::requests::get_transaction_account_metas;
use crate::utils::{
    find_key_indexes, find_unique_account_metas_map, get_program, get_transaction_pubkey,
    get_user_keypair, SQUADS_PROGRAM_ID,
};
use anchor_lang::prelude::AccountMeta;
use solana_sdk::signer::Signer;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use teloxide::types::UserId;

pub fn execute_transaction(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
    user_id: UserId,
) -> Result<Instruction, String> {
    // Retrieve the member's keypair
    let member_keypair = get_user_keypair(user_id);
    let member_pubkey = member_keypair.pubkey();

    // Get the program instance
    let program = get_program(member_keypair, SQUADS_PROGRAM_ID);

    // Get the transaction public key
    let transaction_pubkey = get_transaction_pubkey(multisig_pubkey, transaction_index);

    // Get the account metas associated with the transaction
    let account_metas = get_transaction_account_metas(&program, transaction_pubkey);

    // Find unique account metas
    let unique_account_metas_map = find_unique_account_metas_map(&account_metas);
    let unique_account_metas: Vec<AccountMeta> = unique_account_metas_map.into_values().collect();

    // Find the key indexes for the accounts
    let key_index_array: Vec<u8> = find_key_indexes(&account_metas, &unique_account_metas);

    // Attempt to create the transaction instruction
    program
        .request()
        .accounts(squads_mpl::accounts::ExecuteTransaction {
            multisig: multisig_pubkey,
            transaction: transaction_pubkey,
            member: member_pubkey,
        })
        .args(squads_mpl::instruction::ExecuteTransaction {
            account_list: key_index_array,
        })
        .instructions()
        .and_then(|instructions| instructions.first().cloned()) // Safely retrieve the first instruction
        .map(|mut execute_ix| {
            // Extend the accounts in the instruction
            execute_ix.accounts.extend(unique_account_metas);
            execute_ix
        })
        .ok_or_else(|| "Failed to generate the execute transaction instruction".to_string()) // Handle failure to generate instruction
}
