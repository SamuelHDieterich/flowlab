//! # Base
//! Building blocks to create instructions defined through configuration files (e.g., YAML).

use crate::{from_map_to_vec, from_vec_to_map, Data, DataType, MapKey, Mapify};
use flowlab_macros::{MapKey, Mapify};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Default, PartialEq, Mapify)]
pub struct InstructionCollection(HashMap<PathBuf, InstructionMap>);

impl InstructionCollection {
    pub fn new(filepath: PathBuf, instructions: InstructionMap) -> Self {
        let mut map = HashMap::new();
        map.insert(filepath, instructions);
        Self(map)
    }
}
#[derive(Debug, PartialEq, Mapify, Serialize, Deserialize)]
pub struct InstructionMap(
    #[serde(
        serialize_with = "from_map_to_vec",
        deserialize_with = "from_vec_to_map",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    HashMap<String, Instruction>,
);

/// The Instruction struct is used to define the instructions that a device can perform.
#[derive(Debug, PartialEq, MapKey, Serialize, Deserialize)]
pub struct Instruction {
    #[serde(rename = "instruction")]
    pub name: String,
    pub command: Command,
    #[serde(
        serialize_with = "from_map_to_vec",
        deserialize_with = "from_vec_to_map",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub response: HashMap<String, Response>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub description: String,
}

/// The Command struct is used to define the command that a device can perform.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Command {
    pub query: String,
    #[serde(
        serialize_with = "from_map_to_vec",
        deserialize_with = "from_vec_to_map",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub parameters: HashMap<String, Parameter>,
}

/// The Parameter struct is used to define the parameters that a command can take.
#[derive(Debug, PartialEq, MapKey, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type", default)]
    pub data_type: DataType,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub values: Vec<Data>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub default: Option<Data>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub description: String,
}
/// The Response struct is used to define the response that a command can return.
#[derive(Debug, PartialEq, MapKey, Serialize, Deserialize)]
pub struct Response {
    pub name: String,
    #[serde(rename = "type", default)]
    pub data_type: DataType,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub values: Vec<Data>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub description: String,
}
