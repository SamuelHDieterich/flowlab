/*
 ____
| __ )  __ _ ___  ___
|  _ \ / _` / __|/ _ \
| |_) | (_| \__ \  __/
|____/ \__,_|___/\___|

This submodule has the building blocks to build instructions defined in YAML files.

*/

// Serde: Serialization/Deserialization framework
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The DeviceCommand struct is used to define the instructions that a device can perform.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DeviceCommand {
    #[serde(rename = "instruction")]
    pub name: String,
    pub alias: Option<Vec<String>>,
    pub prelude: Option<Vec<String>>,
    pub command: Command,
    pub response: Option<Vec<Response>>,
    pub description: Option<String>,
}

/// The Command struct is used to define the command that a device can perform.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Command {
    pub query: String,
    pub parameters: Option<Vec<Parameters>>,
}

/// The Parameters struct is used to define the parameters that a command can take.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Parameters {
    pub name: String,
    #[serde(rename = "type", default = "default_data_type")]
    pub data_type: String,
    pub values: Option<Vec<DataType>>,
    pub default: Option<DataType>,
    pub description: Option<String>,
}

/// The Response struct is used to define the response that a command can return.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Response {
    pub attribute: String,
    #[serde(rename = "type", default = "default_data_type")]
    pub data_type: String,
    pub values: Option<Vec<DataType>>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DataType {
    Boolean(bool),
    Integer(i32),
    Float(f32),
    String(String),
}

pub fn default_data_type() -> String {
    "string".to_string()
}

/// Get an instruction from a vector of instructions by its name
#[tracing::instrument]
pub fn find_instruction_with_name<'a>(
    instructions: &'a Vec<DeviceCommand>,
    name: &str,
) -> Option<&'a DeviceCommand> {
    instructions.iter().find(|&i| i.name == name)
}
