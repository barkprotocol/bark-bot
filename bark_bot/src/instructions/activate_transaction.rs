use crate::utils::{get_program, get_transaction_pubkey, get_user_keypair, SQUADS_PROGRAM_ID};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer};
use teloxide::types::UserId;

pub fn activate_transaction(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
    user_id: UserId,
) -> Instruction {
    let creator_keypair = get_user_keypair(user_id);
    let creator_pubkey = creator_keypair.pubkey();
    let program = get_program(creator_keypair, SQUADS_PROGRAM_ID);
    let transaction_pubkey = get_transaction_pubkey(multisig_pubkey, transaction_index);

    return program
        .request()
        .accounts(squads_mpl::accounts::ActivateTransaction {
            multisig: multisig_pubkey,
            transaction: transaction_pubkey,
            creator: creator_pubkey,
        })
        .args(squads_mpl::instruction::ActivateTransaction)
        .instructions()
        .unwrap()
        .first()
        .unwrap()
        .clone();
}
