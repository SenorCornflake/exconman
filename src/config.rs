/// Handles structure of config file

use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub hook_before: Option<String>,
    pub hook_after: Option<String>
}
