/// Handles the structure of a setting

use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Pattern {
    Region([String; 2]),
    Line(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Replace {
    #[serde(rename = "line_above")]
    LineAbove,
    #[serde(rename = "line_below")]
    LineBelow,
    #[serde(rename = "matched_text")]
    MatchedText
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub name: String,
    pub file: String,
    pub pattern: Pattern,
    pub substitute: String,

    pub replace: Option<Replace>, // Default: MatchedText
    pub read_value_path: Option<bool>, // Default: false,
    pub multiple: Option<bool>, // Default: false
    pub before: Option<String>, // Default: false
    pub after: Option<String>, // Default: false
}
