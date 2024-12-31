use std::env;
use teloxide::types::ChatId;
use std::error::Error;

pub fn get_group_chat_id() -> Result<ChatId, Box<dyn Error>> {
    // Try to read the environment variable
    let group_chat_id = env::var("GROUP_CHAT_ID")
        .map_err(|e| format!("Failed to read GROUP_CHAT_ID: {}", e))?;

    // Try to parse the chat ID into an i64
    let chat_id = group_chat_id
        .parse::<i64>()
        .map_err(|e| format!("Failed to parse GROUP_CHAT_ID as i64: {}", e))?;

    Ok(ChatId(chat_id))
}
