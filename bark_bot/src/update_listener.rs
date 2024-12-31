use teloxide::prelude::*;
use teloxide::types::Message;

pub async fn listen_for_updates() {
    let bot = Bot::from_env().auto_send();

    bot.get_updates().await.unwrap().into_iter().for_each(|update| {
        match update {
            Update::Message(message) => {
                // Handle the message
            }
            _ => {}
        }
    });
}
