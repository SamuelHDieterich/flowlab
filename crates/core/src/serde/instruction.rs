//! Serialization and deserialization for instruction-related types.

use serde::{Deserialize, Serialize};

/// A collection of instructions.
type Instructions = Vec<Instruction>;

/// Represents a single instruction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Instruction {
    /// The name of the instruction.
    pub name: String,
    /// The description of the instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The command to be executed.
    pub command: Command,
    /// The expected response from the command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Response>,
}

/// Represents a command to be executed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Command {
    /// The formatted command string.
    pub format: String,
    /// The parameters for the command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
}

/// Represents the expected response from a command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    /// The formatted response string.
    pub format: String,
    /// The parameters for the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
}

/// A collection of parameters.
type Parameters = Vec<Parameter>;

/// Represents a single parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameter {
    /// The name of the parameter.
    pub name: String,
    /// The description of the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The data type of the parameter.
    #[serde(rename = "type", default)]
    pub data_type: ParameterType,
    /// The possible values for the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
    /// The default value for the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

/// Represents the data type of a parameter (default: string).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    #[default]
    String,
    Number,
    Boolean,
}
