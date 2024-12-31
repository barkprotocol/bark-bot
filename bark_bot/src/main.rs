use collections::{Command, Config, Handler, JoinStorage};
use dptree::{case, deps};
use std::path::PathBuf;
use std::sync::Arc;
use teloxide::dispatching::dialogue::serializer::Json;
use teloxide::{
    dispatching::dialogue::{self, ErasedStorage, InMemStorage, SqliteStorage, Storage},
    prelude::*,
};
use dotenv::dotenv;  // Add this to load environment variables

mod actions;
mod collections;
mod commands;
mod handlers;
mod instructions;
mod requests;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();  // Load environment variables from .env file
    
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    // Initialize the bot from the environment variables (TELOXIDE_TOKEN must be set)
    let bot = Bot::from_env();

    // Load configuration
    let config = Config {
        channel_id: Some(-4594739971),
        storage_path: Some(PathBuf::from("db.sqlite")),
    };

    // Initialize storage based on configuration
    let storage: JoinStorage = if let Some(_storage_path) = config.storage_path.clone() {
        // Use SQLite storage if a path is specified
        SqliteStorage::open("db.sqlite", Json)
            .await
            .unwrap()
            .erase()
    } else {
        // Use in-memory storage if no path is specified
        InMemStorage::new().erase()
    };

    // Define the handler for the bot's updates
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

    // Initialize the dispatcher
    Dispatcher::builder(bot, handler)
        .dependencies(deps![storage, Arc::new(config)]) // Pass dependencies (storage and config)
        .default_handler(|_| async move {
            // Handle unknown updates here (ignoring them for now)
        })
        .enable_ctrlc_handler() // Enable the ctrl+c handler to stop the bot gracefully
        .build()
        .dispatch()
        .await; // Start dispatching the bot's updates
}
