//! # Base
//! Building blocks to create instructions defined through configuration files (e.g., YAML).

//----------------//
//---  CRATES  ---//
//----------------//

// Internal modules
use crate::{Data, DataType};

// Built-in modules
//// HashMap: Data structure for storing key-value pairs
use std::collections::HashMap;

//-----------------//
//---  STRUCTS  ---//
//-----------------//

/// The Instruction struct is used to define the instructions that a device can perform.
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub name: String,
    pub description: String,
    pub command: Command,
    pub response: Option<Response>,
}

/// The Parameter struct is used to define the parameters that a command can take or return.
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub data_type: DataType,
    pub values: Vec<Data>,
    pub default: Option<Data>,
    pub description: String,
}

/// The Command struct is used to define the command that a device can perform.
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub query: String,
    pub parameters: HashMap<String, Parameter>,
}

/// The Response struct is used to define the response that a command can return.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Response {
    pub format: String,
    pub parameters: HashMap<String, Parameter>,
}
