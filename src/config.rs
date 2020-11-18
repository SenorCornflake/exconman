use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub registry: String,
    pub modifier: Vec<String>
}

pub fn from_str(json: &str) -> Result<Config, serde_json::Error> {
    let config: Result<Config, serde_json::Error> = serde_json::from_str(json);
    return config;
}

