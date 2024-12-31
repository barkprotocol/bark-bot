use crate::{
    collections::{Handler, HandlerResult, MyDialogue, ParametersData},
    requests::{get_multisig_account, get_transaction_account},
    utils::{
        get_group_chat_id, get_multisig_pubkey, get_transaction_request_buttons,
        get_transaction_request_message,
    },
};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};
use url::form_urlencoded;

pub async fn handle_parameters(
    bot: Bot,
    dialogue: MyDialogue,
    data: ParametersData,
    msg: Message,
) -> HandlerResult {
    // Check if the message contains text
    match msg.text() {
        Some(parameter_value) => {
            let mut parameters_values = data.parameters_values;
            parameters_values.push(parameter_value.to_string());

            let current_param_number = usize::from(data.parameters_number);

            // Check if there are more parameters to be collected
            if current_param_number < data.parameter_names.len() {
                // Ask for the next parameter
                if let Err(e) = bot
                    .send_message(
                        dialogue.chat_id(),
                        data.parameter_labels[current_param_number].to_string(),
                    )
                    .parse_mode(ParseMode::Html)
                    .await
                {
                    eprintln!("Error sending parameter request message: {}", e);
                    return Ok(());
                }

                // Update the dialogue state to move to the next parameter
                let parameters_data = ParametersData {
                    parameter_names: data.parameter_names,
                    parameter_labels: data.parameter_labels,
                    url: data.url,
                    parameters_number: data.parameters_number + 1,
                    parameters_values,
                    action_title: data.action_title,
                    action_description: data.action_description,
                    user_id: data.user_id,
                };

                // Update dialogue with the new state
                if let Err(e) = dialogue
                    .update(Handler::Parameters { data: parameters_data })
                    .await
                {
                    eprintln!("Error updating dialogue state: {}", e);
                    return Ok(());
                }
            } else {
                // All parameters are collected, start building the request URL
                let mut request_url = data.url;

                // Replace placeholders in the URL with the collected parameter values
                for (index, parameter) in data.parameter_names.iter().enumerate() {
                    let item = format!("{{{}}}", parameter);
                    let value = form_urlencoded::byte_serialize(parameters_values[index].as_bytes()).collect::<String>();
                    request_url = request_url.replace(&item, &value);
                }

                // Build a string with the group parameters for the transaction
                let mut group_parameters = String::new();
                for (index, parameter) in data.parameter_labels.iter().enumerate() {
                    group_parameters.push_str(&format!(
                        "<b>{}:</b> {}\n\n",
                        parameter, parameters_values[index]
                    ));
                }

                // Notify the user that processing is happening
                if let Err(e) = bot.send_message(dialogue.chat_id(), "Processing blink...".to_string()).await {
                    eprintln!("Error sending processing message: {}", e);
                    return Ok(());
                }

                // Get multisig public key
                let multsig_pubkey = match get_multisig_pubkey() {
                    Ok(pubkey) => pubkey,
                    Err(e) => {
                        eprintln!("Error getting multisig pubkey: {}", e);
                        bot.send_message(dialogue.chat_id(), "Error retrieving multisig public key. Please try again later.")
                            .await?;
                        return Ok(());
                    }
                };

                // Create the transaction
                let transaction_entry = match crate::actions::create_transaction(&request_url, multsig_pubkey, data.user_id).await {
                    Ok(entry) => entry,
                    Err(e) => {
                        eprintln!("Error creating transaction: {}", e);
                        bot.send_message(dialogue.chat_id(), "Error creating the transaction. Please try again later.")
                            .await?;
                        return Ok(());
                    }
                };

                // Get multisig account and transaction account details
                let multisig_account = match get_multisig_account(multsig_pubkey).await {
                    Ok(account) => account,
                    Err(e) => {
                        eprintln!("Error fetching multisig account: {}", e);
                        bot.send_message(dialogue.chat_id(), "Error fetching multisig account details. Please try again later.")
                            .await?;
                        return Ok(());
                    }
                };

                let threshold = multisig_account.threshold;

                let transaction_account = match get_transaction_account(
                    multsig_pubkey,
                    transaction_entry.transaction_index,
                ).await {
                    Ok(account) => account,
                    Err(e) => {
                        eprintln!("Error fetching transaction account: {}", e);
                        bot.send_message(dialogue.chat_id(), "Error fetching transaction details. Please try again later.")
                            .await?;
                        return Ok(());
                    }
                };

                // Generate the message template and buttons for the group chat
                let template = get_transaction_request_message(
                    data.action_title,
                    data.action_description,
                    Some(group_parameters),
                    transaction_entry.transaction_index,
                );

                let buttons = get_transaction_request_buttons(
                    transaction_entry.id,
                    threshold,
                    1,
                    0,
                    &transaction_account.status,
                );

                // Get the group chat ID and send the message
                let group_chat_id = get_group_chat_id();
                let group_message = match bot
                    .send_message(group_chat_id, template)
                    .parse_mode(ParseMode::Html)
                    .reply_markup(InlineKeyboardMarkup::new([buttons]))
                    .await {
                    Ok(message) => message,
                    Err(e) => {
                        eprintln!("Error sending group message: {}", e);
                        bot.send_message(dialogue.chat_id(), "Error notifying the group. Please try again later.")
                            .await?;
                        return Ok(());
                    }
                };

                // Update the transaction with the group message ID
                crate::requests::update_transaction(transaction_entry.id, group_message.id).await;

                // Notify the user that the transaction has been sent
                if let Err(e) = bot.send_message(msg.chat.id, "Transaction sent!".to_string()).await {
                    eprintln!("Error sending success message: {}", e);
                }

                // Exit the dialogue as the process is complete
                if let Err(e) = dialogue.exit().await {
                    eprintln!("Error exiting dialogue: {}", e);
                    return Ok(());
                }
            }
        }
        _ => {
            // Handle cases where the user didn't send plain text
            if let Err(e) = bot.send_message(msg.chat.id, "Please send plain text.").await {
                eprintln!("Error sending text message: {}", e);
            }
        }
    }

    Ok(())
}
