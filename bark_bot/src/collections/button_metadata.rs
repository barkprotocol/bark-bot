use std::convert::{TryFrom, From};

#[derive(Debug)]
pub struct InvalidButtonMetadataError {}

#[derive(Debug)]
pub struct ButtonMetadata {
    pub transaction_id: i64,
    pub value: String,
}

impl From<&ButtonMetadata> for String {
    fn from(transaction_data: &ButtonMetadata) -> Self {
        let result = format!(
            "{},{}",
            transaction_data.transaction_id, transaction_data.value,
        );

        result
    }
}

impl TryFrom<String> for ButtonMetadata {
    type Error = InvalidButtonMetadataError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(',').collect();
        if parts.len() != 2 {
            return Err(InvalidButtonMetadataError {});
        }

        let transaction_id = parts[0]
            .parse::<i64>()
            .map_err(|_| InvalidButtonMetadataError {})?;

        let value = String::from(parts[1].trim());

        Ok(ButtonMetadata {
            transaction_id,
            value,
        })
    }
}

fn main() {
    // Create a ButtonMetadata instance
    let metadata = ButtonMetadata {
        transaction_id: 12345,
        value: "Transaction Value".to_string(),
    };

    // Convert ButtonMetadata to String
    let metadata_string: String = String::from(&metadata);
    println!("Serialized ButtonMetadata: {}", metadata_string);

    // Parse the String back to ButtonMetadata
    let parsed_metadata: Result<ButtonMetadata, InvalidButtonMetadataError> = ButtonMetadata::try_from(metadata_string);
    
    match parsed_metadata {
        Ok(parsed) => {
            println!("Parsed ButtonMetadata: {:?}", parsed);
        },
        Err(_) => {
            println!("Failed to parse the ButtonMetadata from the string.");
        }
    }
}
