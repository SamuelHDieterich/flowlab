//! # Base
//! Building blocks to create instructions defined through configuration files (e.g., YAML).

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

/// The DataType enum is used to define the data types that a parameter or response can take.
/// It is used to define the data type of the parameter or response.
/// The order of the variants is important for the deserialization process.
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
///
/// # Arguments
///
/// - `instructions`
///   A `Vec` of `DeviceCommand` with the instructions.
/// - `name`
///   A `&str` with the name of the instruction to be found.
///
/// # Returns
///
/// A reference to the `DeviceCommand` with the instruction found.
///
/// # Example
///
/// ```
/// use flowlab::instruction::base::{Parameters, Command, DeviceCommand, find_instruction_with_name};
///
/// let instructions = vec![
///    DeviceCommand {
///        name: "echo".to_string(),
///        alias: None,
///        prelude: None,
///        command: Command {
///            query: "echo {{ param1 }} {{ param2 }}".to_string(),
///            parameters: Some(vec![
///                Parameters {
///                    name: "param1".to_string(),
///                    data_type: "string".to_string(),
///                    values: None,
///                    default: None,
///                    description: None,
///                },
///                Parameters {
///                    name: "param2".to_string(),
///                    data_type: "string".to_string(),
///                    values: None,
///                    default: None,
///                    description: None,
///                },
///            ])
///        },
///        response: None,
///        description: None,
///    },
/// ];
/// let instruction = find_instruction_with_name(&instructions, "echo");
/// ```
#[tracing::instrument]
pub fn find_instruction_with_name<'a>(
    instructions: &'a Vec<DeviceCommand>,
    name: &str,
) -> Option<&'a DeviceCommand> {
    instructions.iter().find(|&i| i.name == name)
}

impl std::fmt::Display for DeviceCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let serialized = match serde_yaml::to_string(self) {
            Ok(s) => s,
            Err(_) => return Err(std::fmt::Error),
        };
        write!(f, "{}", serialized)
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let serialized = match serde_yaml::to_string(self) {
            Ok(s) => s,
            Err(_) => return Err(std::fmt::Error),
        };
        write!(f, "{}", serialized)
    }
}

impl std::fmt::Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let serialized = match serde_yaml::to_string(self) {
            Ok(s) => s,
            Err(_) => return Err(std::fmt::Error),
        };
        write!(f, "{}", serialized)
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let serialized = match serde_yaml::to_string(self) {
            Ok(s) => s,
            Err(_) => return Err(std::fmt::Error),
        };
        write!(f, "{}", serialized)
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let serialized = match serde_yaml::to_string(self) {
            Ok(s) => s,
            Err(_) => return Err(std::fmt::Error),
        };
        write!(f, "{}", serialized)
    }
}