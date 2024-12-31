use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    // Optional channel_id of type i64
    pub channel_id: Option<i64>,
    
    // Optional storage_path of type PathBuf to represent the file path
    pub storage_path: Option<PathBuf>,
}
