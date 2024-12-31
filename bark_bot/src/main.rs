use squads_mpl::state::MsTransactionStatus;
use teloxide::{dispatching::dialogue::{self, ErasedStorage, InMemStorage, SqliteStorage}, prelude::*};
use dptree::{case, deps};
use std::{env, path::PathBuf, sync::Arc};
use teloxide::{types::Update};

mod actions;
mod collections;
mod commands;
mod handlers;
mod instructions;
mod requests;
mod utils;

#[tokio::main]
async fn main() {
    // Initialize logging with pretty_env_logger for better debugging
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    // Load the bot token from environment variables (ensure to set the token in your environment)
    let bot_token = env::var("TELOXIDE_TOKEN")
        .expect("TELOXIDE_TOKEN must be set in the environment");
    let bot = Bot::new(bot_token);

    // Load configuration from environment variables or use default
    let config = Config {
        channel_id: Some(-4594739971), // Example channel ID
        storage_path: env::var("STORAGE_PATH").ok().map(PathBuf::from), // Dynamically use environment variable
    };

    // Configure the storage: SQLite or In-memory storage depending on the environment
    let storage: JoinStorage = if let Some(storage_path) = config.storage_path.clone() {
        match SqliteStorage::open(storage_path, Json).await {
            Ok(storage) => storage.erase(),  // Use SQLite if it exists
            Err(e) => {
                log::error!("Failed to open SQLite storage: {}", e);
                InMemStorage::new().erase() // Fall back to in-memory storage on failure
            }
        }
    } else {
        InMemStorage::new().erase() // Default to in-memory storage if no storage path provided
    };

    // Define the handler for processing incoming updates from users
    let handler = dialogue::enter::<Update, ErasedStorage<Handler>, Handler, _>()
        .branch(Update::filter_callback_query().branch(
            case![Handler::InternalAction { data }].endpoint(handlers::handle_internal_action),
        ))
        .branch(Update::filter_callback_query().endpoint(handlers::handle_external_action))
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, ErasedStorage<Handler>, Handler>()
                .branch(case![Handler::BlinkUrl].endpoint(handlers::handle_blink_url))
                .branch(case![Handler::Parameters { data }].endpoint(handlers::handle_parameters)),
        )
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .branch(case![Command::Help].endpoint(commands::help))
                .branch(case![Command::Cancel].endpoint(commands::cancel)),
        );

    // Set up the dispatcher to process updates and start listening for incoming messages
    Dispatcher::builder(bot, handler)
        .dependencies(deps![storage, Arc::new(config)])  // Pass storage and config as dependencies
        .default_handler(|_| async move {
            log::warn!("Received unknown update. Ignoring.");
        })
        .enable_ctrlc_handler() // Gracefully handle Ctrl+C
        .build()
        .dispatch()
        .await;
}
