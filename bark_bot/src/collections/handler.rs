use crate::collections::Action;
use serde::{Deserialize, Serialize};
use teloxide::types::UserId;

// Struct to store internal action data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InternalActionData {
    pub actions: Vec<Action>,            // List of actions to be performed
    pub base_url: String,                // Base URL for constructing full URLs
    pub url: String,                     // URL associated with the specific action
    pub action_title: String,            // Title of the action
    pub action_description: String,      // Description of the action
    pub user_id: UserId,                 // The user who initiated the action
}

// Struct to store parameters related to an action
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParametersData {
    pub parameter_names: Vec<String>,     // Names of the parameters for the action
    pub parameter_labels: Vec<String>,    // Labels for the parameters to display to users
    pub url: String,                      // URL to make requests or handle the action
    pub parameters_number: u8,            // The number of parameters for the action
    pub parameters_values: Vec<String>,   // Values corresponding to the parameters
    pub action_title: String,             // Title of the action
    pub action_description: String,       // Description of the action
    pub user_id: UserId,                  // User ID related to the action
}

// Enum to handle different types of user interaction states
#[derive(Clone, Default, Deserialize, Serialize)]
pub enum Handler {
    #[default]  // Default state, likely for Blink URL
    BlinkUrl, 
    InternalAction {  // Internal action state, contains data for an internal action
        data: InternalActionData,
    },
    Parameters {  // Parameters state, contains data for action parameters
        data: ParametersData,
    },
}

