use std::collections::BTreeMap;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ValueType {
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "text")]
    Text
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MatchType {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "region")]
    Region
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ReplaceLine {
    #[serde(rename = "above")]
    Above,
    #[serde(rename = "below")]
    Below,
    #[serde(rename = "matched")]
    Matched
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Setting {
    pub file: String,
    pub value_type: ValueType,
    pub replace_with: String,
    pub replace_line: ReplaceLine,
    pub match_pattern: String,
    pub match_type: MatchType
}

pub fn from_json(json: &str) -> Result<BTreeMap<String, Setting>, serde_json::Error> {
    let registry: Result<BTreeMap<String, Setting>, serde_json::Error> = serde_json::from_str(json);
    return registry;
}
