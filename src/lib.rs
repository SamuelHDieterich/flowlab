//! # flowlab
//!
//! `flowlab` is a library for the program with same name used for control and monitoring system of
//! laboratory instruments.
//!
//! This is a term project for the course "Engineering Physics" at the Universidade Federal do Rio
//! Grande do Sul (UFRGS). The project was develop to work on a specific equipment, the C-MAG 9
//! Cryostat, but this project is designed to be used with any other equipment.

//-----------------//
//---  MODULES  ---//
//-----------------//

// Device module
pub mod device;

// Instruction module
pub mod instruction;

// Pipeline module
pub mod pipeline;

//----------------//
//---  CRATES  ---//
//----------------//

// External crates
//// Serde: Serialization/Deserialization framework
use serde::{Deserialize, Serialize};

//---------------//
//---  ENUMS  ---//
//---------------//

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Boolean,
    Integer,
    Float,
    String,
}

/// The Data enum is used to define the data types that a parameter or response can take.
/// It is used to define the data type of the parameter or response.
/// The order of the variants is important for the deserialization process.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Data {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

//-------------------------//
//---  IMPLEMENTATIONS  ---//
//-------------------------//

impl Default for Data {
    fn default() -> Self {
        Data::String(String::default())
    }
}

impl Default for DataType {
    fn default() -> Self {
        DataType::String
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Boolean(value) => write!(f, "{}", value),
            Data::Integer(value) => write!(f, "{}", value),
            Data::Float(value) => write!(f, "{}", value),
            Data::String(value) => write!(f, "{}", value),
        }
    }
}
