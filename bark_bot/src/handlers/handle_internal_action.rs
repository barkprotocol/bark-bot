use crate::collections::{Handler, HandlerResult, InternalActionData, MyDialogue, ParametersData};
use crate::requests::{get_multisig_account, get_transaction_account};
use crate::utils::{
    get_group_chat_id, get_multisig_pubkey, get_transaction_request_buttons,
    get_transaction_request_message, get_url_root,
};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

pub async fn handle_internal_action(
    bot: Bot,
    dialogue: MyDialogue,
    data: InternalActionData,
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(action_name) = &q.data {
        let action = data.actions.iter().find(|&a| a.label == *action_name);

        match action {
            Some(res) => {
                let parameters = res.parameters.clone();

                match parameters {
                    Some(parameters_res) => {
                        if parameters_res.is_empty() {
                            // Handle action immediately if no parameters
                            if let Err(e) = bot.send_message(dialogue.chat_id(), "Processing blink...".to_string()).await {
                                eprintln!("Error sending message: {}", e);
                                return Ok(());
                            }

                            let action_url = format!("{}{}", data.base_url, action.unwrap().href);
                            let multisig_pubkey = match get_multisig_pubkey() {
                                Ok(pubkey) => pubkey,
                                Err(e) => {
                                    eprintln!("Error fetching multisig pubkey: {}", e);
                                    bot.send_message(dialogue.chat_id(), "Failed to retrieve multisig pubkey. Try again later.")
                                        .await?;
                                    return Ok(());
                                }
                            };

                            let transaction_entry = match crate::actions::create_transaction(
                                &action_url,
                                multisig_pubkey,
                                data.user_id,
                            ).await {
                                Ok(entry) => entry,
                                Err(e) => {
                                    eprintln!("Error creating transaction: {}", e);
                                    bot.send_message(dialogue.chat_id(), "Failed to create transaction. Try again later.")
                                        .await?;
                                    return Ok(());
                                }
                            };

                            let multisig_account = match get_multisig_account(multisig_pubkey).await {
                                Ok(account) => account,
                                Err(e) => {
                                    eprintln!("Error fetching multisig account: {}", e);
                                    bot.send_message(dialogue.chat_id(), "Failed to fetch multisig account. Try again later.")
                                        .await?;
                                    return Ok(());
                                }
                            };

                            let threshold = multisig_account.threshold;
                            let transaction_account = match get_transaction_account(
                                multisig_pubkey,
                                transaction_entry.transaction_index,
                            ).await {
                                Ok(account) => account,
                                Err(e) => {
                                    eprintln!("Error fetching transaction account: {}", e);
                                    bot.send_message(dialogue.chat_id(), "Failed to retrieve transaction details. Try again later.")
                                        .await?;
                                    return Ok(());
                                }
                            };

                            let template = get_transaction_request_message(
                                data.action_title,
                                data.action_description,
                                None,
                                transaction_entry.transaction_index,
                            );

                            let buttons = get_transaction_request_buttons(
                                transaction_entry.id,
                                threshold,
                                1,
                                0,
                                &transaction_account.status,
                            );

                            let group_chat_id = get_group_chat_id();
                            let group_message = match bot
                                .send_message(group_chat_id, template)
                                .parse_mode(ParseMode::Html)
                                .reply_markup(InlineKeyboardMarkup::new([buttons]))
                                .await {
                                    Ok(message) => message,
                                    Err(e) => {
                                        eprintln!("Error sending group message: {}", e);
                                        bot.send_message(dialogue.chat_id(), "Failed to notify group. Try again later.")
                                            .await?;
                                        return Ok(());
                                    }
                                };

                            crate::requests::update_transaction(transaction_entry.id, group_message.id)
                                .await;

                            bot.send_message(dialogue.chat_id(), "Transaction sent!".to_string())
                                .await?;

                            dialogue.exit().await?;
                        } else {
                            // Validate parameters
                            let parameter_names: Vec<String> =
                                parameters_res.iter().map(|p| p.name.clone()).collect();
                            let parameter_labels: Vec<String> =
                                parameters_res.iter().map(|p| p.label.clone()).collect();
                            let mut result_parameters = String::new();

                            for (index, parameter) in parameters_res.iter().enumerate() {
                                match parameter.required {
                                    Some(is_required) => {
                                        result_parameters.push_str(&format!(
                                            "{} - {} ({})\n\n",
                                            index + 1,
                                            parameter.label,
                                            if is_required { "Required" } else { "Optional" }
                                        ));
                                    }
                                    None => {
                                        result_parameters.push_str(&format!(
                                            "{} - {} (Optional)\n\n",
                                            index + 1,
                                            parameter.label
                                        ));
                                    }
                                }
                            }

                            // Prompt for missing parameters
                            if let Err(e) = bot.send_message(dialogue.chat_id(), format!(
                                "The action <b>{}</b> has the following params: \n\n{}Please enter the values as they are requested.", 
                                action_name, result_parameters
                                )
                            ).parse_mode(ParseMode::Html).await {
                                eprintln!("Error sending parameter message: {}", e);
                                return Ok(());
                            }

                            if let Err(e) = bot.send_message(dialogue.chat_id(), parameter_labels[0].to_string())
                                .parse_mode(ParseMode::Html)
                                .await {
                                    eprintln!("Error asking for first parameter: {}", e);
                                    return Ok(());
                            }

                            let parameters_values: Vec<String> = Vec::new();

                            let parameters_data = ParametersData {
                                parameter_names,
                                parameter_labels,
                                url: format!("{}{}", get_url_root(&data.url).unwrap(), res.href),
                                parameters_number: 1,
                                parameters_values,
                                action_title: data.action_title,
                                action_description: data.action_description,
                                user_id: data.user_id,
                            };

                            dialogue
                                .update(Handler::Parameters {
                                    data: parameters_data,
                                })
                                .await?;
                        }
                    }
                    None => {
                        // Handle case where parameters are missing or not found
                        eprintln!("Parameters are missing or incorrect for action.");
                        bot.send_message(dialogue.chat_id(), "Parameters for the action could not be found.")
                            .await?;
                    }
                }
            }
            None => {
                bot.send_message(dialogue.chat_id(), "Send me a valid action.")
                    .await?;
            }
        }
    }

    Ok(())
}
