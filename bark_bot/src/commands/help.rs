use crate::collections::{Command, HandlerResult};
use teloxide::{prelude::*, utils::command::BotCommands};

// Function to handle the "help" command
pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    // Send the descriptions of all supported commands to the user.
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;

    // Return Ok to indicate successful handling of the command.
    Ok(())
}
