use crate::collections::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

// Function to handle the "cancel" command in the dialogue
pub async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    // Send a message to the user notifying that the Blink request is being canceled.
    bot.send_message(msg.chat.id, "Cancelling the Blink request.")
        .await?;

    // Exit the current dialogue, effectively ending the conversation state for this user.
    dialogue.exit().await?;

    // Return Ok to indicate successful handling of the command.
    Ok(())
}
