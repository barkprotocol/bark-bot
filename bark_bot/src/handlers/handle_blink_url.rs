use crate::{
    collections::{Handler, HandlerResult, InternalActionData, MyDialogue},
    requests::{get_blink_metadata, get_blink_transaction},
    utils::get_multisig_pubkey,
};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode},
};
use url::Url;

pub async fn handle_blink_url(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(url) => {
            // Fetch Blink metadata for the given URL
            let response = get_blink_metadata(&url.to_string()).await;
            let parsed_url = Url::parse(url)?;
            let base_url = format!("{}://{}", parsed_url.scheme(), parsed_url.host().unwrap());

            match response {
                Ok(res) => {
                    // Handle the links in the Blink metadata
                    let links = res.links;

                    match links {
                        Some(links_res) => {
                            // Extract actions and generate action buttons
                            let actions = links_res.actions;
                            let action_names: Vec<String> =
                                actions.iter().map(|a| a.label.clone()).collect();
                            let action_buttons = action_names
                                .iter()
                                .map(|action| InlineKeyboardButton::callback(action, action));

                            // Generate the message template to send to the user
                            let template = format!(
                                "<b>{}</b> \n\n{} \n\n{}\n\n Choose an action to perform:",
                                res.title, res.description, res.icon
                            );

                            // Send the message with an inline keyboard
                            bot.send_message(msg.chat.id, template)
                                .parse_mode(ParseMode::Html)
                                .reply_markup(InlineKeyboardMarkup::new([action_buttons]))
                                .await?;

                            // Update the dialogue with the action data
                            let actions_data = InternalActionData {
                                actions,
                                url: url.to_string(),
                                action_title: res.title,
                                action_description: res.description,
                                user_id: msg.from.clone().unwrap().id,
                                base_url,
                            };

                            dialogue
                                .update(Handler::InternalAction { data: actions_data })
                                .await?;
                        }
                        None => {
                            // If no actions are available, fetch Blink transaction
                            let multisig_pubkey = get_multisig_pubkey();
                            let transaction_response =
                                get_blink_transaction(multisig_pubkey, &url.to_string()).await?;

                            // Generate the message template with the transaction details
                            let template = format!(
                                "<b>{}</b> \n\n{} \n\n{} \n\n{}",
                                res.title,
                                res.description,
                                res.icon,
                                transaction_response.transaction
                            );

                            // Send the transaction details to the user
                            bot.send_message(msg.chat.id, template)
                                .parse_mode(ParseMode::Html)
                                .await?;

                            // Exit the dialogue
                            dialogue.exit().await?;
                        }
                    }
                }
                Err(e) => {
                    // If there is an error fetching the Blink metadata, notify the user
                    bot.send_message(msg.chat.id, format!("Error getting blink: {}", e))
                        .await?;
                }
            }
        }
        None => {
            // If no text (URL) is provided, ask the user for a valid Blink URL
            bot.send_message(msg.chat.id, "Please, share with me a valid Blink URL")
                .await?;
        }
    }

    Ok(())
}
