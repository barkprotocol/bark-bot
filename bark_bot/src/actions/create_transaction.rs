use crate::{
    collections::Transaction,
    requests::{get_blink_transaction, get_multisig_account, send_and_confirm_transaction},
    utils::{find_blink_instructions, get_user_keypair},
};
use solana_sdk::{
    instruction::Instruction, message::Message, pubkey::Pubkey, signature::Keypair, signer::Signer,
};
use teloxide::types::UserId;

pub async fn create_transaction(
    url: &str,
    multisig_pubkey: Pubkey,
    user_id: UserId,
) -> Result<Transaction, String> {  // Return Result for error handling
    // Get the multisig account
    let multisig_account = match get_multisig_account(multisig_pubkey).await {
        Ok(account) => account,
        Err(e) => return Err(format!("Failed to get multisig account: {}", e)),
    };
    
    let transaction_index = multisig_account.transaction_index + 1;

    // Collect instructions
    let mut instructions: Vec<Instruction> = vec![];

    // Add the transaction creation instruction
    instructions.push(crate::instructions::create_transaction(
        multisig_pubkey,
        transaction_index,
        user_id,
    ));

    let mut instruction_index: u8 = 1;

    // Fetch Blink transaction instructions
    let get_blink_transaction_response = match get_blink_transaction(multisig_pubkey, url).await {
        Ok(response) => response,
        Err(e) => return Err(format!("Failed to get Blink transaction: {}", e)),
    };

    // Find instructions from Blink response
    let blink_instructions = match find_blink_instructions(get_blink_transaction_response.transaction) {
        Ok(instructions) => instructions,
        Err(e) => return Err(format!("Failed to find Blink instructions: {}", e)),
    };

    // Add Blink instructions to the transaction
    for instruction in blink_instructions {
        instructions.push(crate::instructions::add_instruction(
            multisig_pubkey,
            transaction_index,
            instruction_index,
            instruction,
            user_id,
        ));
        instruction_index += 1;
    }

    // Add the activation and approval instructions
    instructions.push(crate::instructions::activate_transaction(
        multisig_pubkey,
        transaction_index,
        user_id,
    ));

    instructions.push(crate::instructions::approve_transaction(
        multisig_pubkey,
        transaction_index,
        user_id,
    ));

    // Get the creator's keypair
    let creator_keypair = match get_user_keypair(user_id).await {
        Ok(keypair) => keypair,
        Err(e) => return Err(format!("Failed to get user keypair: {}", e)),
    };
    
    let creator_pubkey = creator_keypair.pubkey();
    let message = Message::new(&instructions, Some(&creator_pubkey));
    
    let signers: Vec<&Keypair> = vec![&creator_keypair];

    // Send and confirm the transaction
    let signature = match send_and_confirm_transaction(message, signers).await {
        Ok(sig) => sig,
        Err(e) => return Err(format!("Failed to send and confirm transaction: {}", e)),
    };

    // Call create_transaction API and return the result
    match crate::requests::create_transaction(
        transaction_index.try_into().unwrap(),  // Consider handling this safely
        user_id,
        signature.to_string(),
    )
    .await {
        Ok(transaction) => Ok(transaction),
        Err(e) => Err(format!("Failed to create transaction: {}", e)),
    }
}
