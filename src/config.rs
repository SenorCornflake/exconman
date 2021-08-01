/// Handles structure of config file

use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub keys: HashMap<String, String>
}
