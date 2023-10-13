use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    #[serde(rename = "instruction")]
    pub name: String,
    pub alias: Option<Vec<String>>,
    pub prelude: Option<Vec<String>>,
    pub command: Command,
    pub response: Option<Vec<Response>>,
    pub description: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub query: String,
    pub parameters: Option<Vec<Parameters>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Parameters {
    pub name: String,
    #[serde(rename = "type", default = "default_data_type")]
    pub data_type: String,
    pub values: Option<Vec<String>>,
    pub default: Option<String>,
    pub description: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub attribute: String,
    #[serde(rename = "type", default = "default_data_type")]
    pub data_type: String,
    pub values: Option<Vec<String>>,
    pub description: Option<String>,
}

pub fn default_data_type() -> String {
    "string".to_string()
}
