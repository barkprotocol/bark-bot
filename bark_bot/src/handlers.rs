use teloxide::prelude::*;
use teloxide::types::{Message, User};
use crate::{actions::approve_transaction, actions::execute_transaction, actions::reject_transaction};
use solana_sdk::pubkey::Pubkey;
use teloxide::types::UserId;
use crate::models::BlinkMetadata;

pub async fn handle_message(message: Message) {
    let bot = Bot::from_env().auto_send();

    // Respond to different types of messages or commands
    if let Some(text) = message.text() {
        match text {
            "/start" => handle_start_command(message).await,
            "/approve" => handle_approve(message).await,
            "/execute" => handle_execute(message).await,
            "/reject" => handle_reject(message).await,
            _ => handle_unknown_command(message).await,
        }
    }
}

async fn handle_start_command(message: Message) {
    let bot = Bot::from_env().auto_send();
    let user = message.from().unwrap();

    // Send a welcome message or instructions
    bot.send_message(message.chat.id, format!("Welcome, {}!", user.first_name))
        .await
        .unwrap();
}

async fn handle_approve(message: Message) {
    let user_id = message.from().unwrap().id;
    let multisig_pubkey = Pubkey::from_str("YourMultisigPubkeyHere").unwrap();
    let transaction_index = 0; // Replace with actual transaction index

    match approve_transaction(multisig_pubkey, transaction_index, user_id).await {
        Ok(signature) => {
            let bot = Bot::from_env().auto_send();
            bot.send_message(message.chat.id, format!("Transaction approved! Signature: {}", signature))
                .await
                .unwrap();
        }
        Err(error) => {
            let bot = Bot::from_env().auto_send();
            bot.send_message(message.chat.id, format!("Error approving transaction: {}", error))
                .await
                .unwrap();
        }
    }
}

async fn handle_execute(message: Message) {
    let user_id = message.from().unwrap().id;
    let multisig_pubkey = Pubkey::from_str("YourMultisigPubkeyHere").unwrap();
    let transaction_index = 1; // Replace with actual transaction index

    match execute_transaction(multisig_pubkey, transaction_index, user_id).await {
        Ok(signature) => {
            let bot = Bot::from_env().auto_send();
            bot.send_message(message.chat.id, format!("Transaction executed! Signature: {}", signature))
                .await
                .unwrap();
        }
        Err(error) => {
            let bot = Bot::from_env().auto_send();
            bot.send_message(message.chat.id, format!("Error executing transaction: {}", error))
                .await
                .unwrap();
        }
    }
}

async fn handle_reject(message: Message) {
    let user_id = message.from().unwrap().id;
    let multisig_pubkey = Pubkey::from_str("2NTvEssJ2i998V2cMGT4Fy3JhyFnAzHFonDo9dbAkVrg").unwrap();
    let transaction_index = 2; // Replace with actual transaction index

    match reject_transaction(multisig_pubkey, transaction_index, user_id).await {
        Ok(signature) => {
            let bot = Bot::from_env().auto_send();
            bot.send_message(message.chat.id, format!("Transaction rejected! Signature: {}", signature))
                .await
                .unwrap();
        }
        Err(error) => {
            let bot = Bot::from_env().auto_send();
            bot.send_message(message.chat.id, format!("Error rejecting transaction: {}", error))
                .await
                .unwrap();
        }
    }
}

async fn handle_unknown_command(message: Message) {
    let bot = Bot::from_env().auto_send();
    bot.send_message(message.chat.id, "Unknown command. Please try again.")
        .await
        .unwrap();
}

async fn send_blink_metadata(message: Message, metadata: BlinkMetadata) {
    let bot = Bot::from_env().auto_send();

    // Send Blink metadata info to the user
    bot.send_message(message.chat.id, format!(
        "Icon: {}\nTitle: {}\nDescription: {}\nLabel: {}\n",
        metadata.icon,
        metadata.title,
        metadata.description,
        metadata.label
    ))
    .await
    .unwrap();

    if let Some(links) = metadata.links {
        // Process links and send them
        for action in links.actions {
            bot.send_message(
                message.chat.id,
                format!("Action: {}\nLink: {}", action.label, action.href)
            ).await.unwrap();
        }
    }
}
