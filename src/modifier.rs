use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ValueType {
    Bool(bool),
    String(String)
}

pub fn from_str(json: &str) -> Result<HashMap<String, ValueType>, serde_json::Error> {
    let modifier: Result<HashMap<String, ValueType>, serde_json::Error> = serde_json::from_str(json);
    return modifier;
}

