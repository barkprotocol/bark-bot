use serde::{Deserialize, Serialize};

/// Represents a parameter for an action, such as a user input for a particular action.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub label: String,        // The label for the parameter (e.g., display name).
    pub name: String,         // The internal name of the parameter (used in code or API).
    pub required: Option<bool>, // Indicates whether the parameter is required.
}

/// Represents an action that can be performed, with associated parameters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub label: String,        // The label describing the action.
    pub href: String,         // The URL or endpoint for the action.
    pub parameters: Option<Vec<Parameter>>, // Optional list of parameters for the action.
}

/// Represents a collection of actions that can be associated with Blink metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub actions: Vec<Action>, // A list of actions for the metadata item.
}

/// Represents the full metadata for a Blink item, including actions, description, and other details.
#[derive(Debug, Deserialize)]
pub struct BlinkMetadata {
    pub icon: String,        // URL or path to the icon image.
    pub title: String,       // The title of the Blink item.
    pub description: String, // A description of the Blink item.
    pub label: String,       // A label for the Blink item.
    pub links: Option<Links>, // Optional links with actions.
    pub disabled: Option<bool>, // Indicates if the Blink item is disabled.
    pub error: Option<bool>, // Indicates if there was an error with the Blink item.
}

impl BlinkMetadata {
    // Example method to get all actions associated with Blink metadata
    pub fn get_actions(&self) -> Option<Vec<Action>> {
        self.links.as_ref().map(|links| links.actions.clone())
    }
}
