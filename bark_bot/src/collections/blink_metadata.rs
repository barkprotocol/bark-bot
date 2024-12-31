use serde::{Deserialize, Serialize};

/// Represents a parameter in an action
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub label: String,     // The label for the parameter
    pub name: String,      // The name of the parameter
    pub required: Option<bool>, // Indicates if the parameter is required, or None if unspecified
}

/// Represents an action with parameters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub label: String,         // The label for the action
    pub href: String,          // The URL or link associated with the action
    pub parameters: Option<Vec<Parameter>>, // Optional list of parameters required for the action
}

/// Represents a set of links related to actions
#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub actions: Vec<Action>, // List of actions that this link supports
}

/// Represents Blink Metadata, typically for a Blink interface
#[derive(Debug, Serialize, Deserialize)]
pub struct BlinkMetadata {
    pub icon: String,            // Icon representing the Blink entity
    pub title: String,           // Title of the Blink entity
    pub description: String,     // Description for the Blink entity
    pub label: String,           // Label associated with the Blink entity
    pub links: Option<Links>,    // Optional links to associated actions
    pub disabled: Option<bool>,  // Whether the entity is disabled; None means unspecified
    pub error: Option<bool>,     // Whether the entity has an error; None means unspecified
}
