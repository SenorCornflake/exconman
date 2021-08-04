/// Handles structure of config file

use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub hook_before_get: Option<String>,
    pub hook_after_get: Option<String>,
    pub hook_before_set: Option<String>,
    pub hook_after_set: Option<String>
}
