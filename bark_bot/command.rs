use teloxide::{prelude::*, types::Message};

pub async fn start_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let start_message = "Welcome to the BARK Bot!\n\n\
                         BARK Bot simplifies multi-signature transactions and integrates with Dialect's Blink protocol for seamless treasury management and decision-making. Here's what you can do:\n\n\
                         - Approve or reject transactions\n\
                         - Manage treasury operations\n\
                         - Execute group decisions\n\n\
                         Type /help to learn more about the commands available.";

    bot.send_message(msg.chat.id, start_message).await?;
    Ok(())
}
