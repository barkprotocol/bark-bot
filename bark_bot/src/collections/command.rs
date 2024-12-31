use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::sync::mpsc;

// Define the commands available to users
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase", 
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Shows all supported commands")]
    Help,
    #[command(description = "Initialize the flow.")]
    Start,
    #[command(description = "Cancel the Blink request.")]
    Cancel,
}

// Structure to manage the flow state
#[derive(Debug, Default)]
pub struct FlowState {
    pub is_in_progress: bool,
}

// Function to handle each command based on user input
async fn handle_command(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
    flow_state: &mut FlowState,
    tx: mpsc::Sender<String>, // Sender for communication with other parts of the app
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match command {
        Command::Help => {
            // Respond with the help message listing all commands
            cx.answer("These are the available commands:\n/start - Initialize the flow\n/cancel - Cancel the Blink request\n/help - Shows this help message").send().await?;
        },
        Command::Start => {
            // Logic to start the flow
            if flow_state.is_in_progress {
                cx.answer("A flow is already in progress. Please cancel it before starting a new one.").send().await?;
            } else {
                flow_state.is_in_progress = true;
                // Send message to notify the system of the flow start
                tx.send("Flow started".to_string()).await?;
                cx.answer("Starting the flow...").send().await?;
            }
        },
        Command::Cancel => {
            // Logic to cancel the flow
            if flow_state.is_in_progress {
                flow_state.is_in_progress = false;
                // Send message to notify the system that the flow has been canceled
                tx.send("Flow canceled".to_string()).await?;
                cx.answer("Blink request has been canceled.").send().await?;
            } else {
                cx.answer("No flow is in progress to cancel.").send().await?;
            }
        },
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize bot and communication channels
    let bot = Bot::from_env();
    let (tx, mut rx) = mpsc::channel::<String>(100);  // Channel to handle messages between the bot and other parts of the app
    let mut flow_state = FlowState::default();  // State to track the flow progress

    // Start a task to listen for messages and perform actions based on them
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("Message received: {}", message);
            // Logic to handle different messages (e.g., start flow, cancel flow)
        }
    });

    // Start the Teloxide command handler
    teloxide::commands_repl(bot, "blink_bot", move |cx, command| {
        let tx = tx.clone();
        let flow_state = flow_state.clone();
        async move {
            handle_command(cx, command, &mut flow_state, tx).await?;
            Ok(())
        }
    })
    .await;
}
