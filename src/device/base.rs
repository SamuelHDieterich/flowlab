//! # Base
//! Base building blocks for creating devices with generic protocols.
//!
//! With this module, you can create `Device`s with any `Protocol` that implements the `Query` trait.
//! - `Device`: Physical or virtual object that can be controlled or monitored by the system.
//! - `Protocol`: Communication protocol that the device can communicate with, for instance, `TCP` and `Serial`.
//! - `Query`: Trait that allows the device to send commands and receive responses.

//-----------------//
//---  MODULES  ---//
//-----------------//

// Internal modules
use super::Query;
use crate::{instruction::Instruction, Data};

// Built-in modules
//// Basic data structures
use std::collections::HashMap;

// External crates
//// Serde: Serialization/Deserialization framework
use serde::{de::DeserializeOwned, Serialize};

//-----------------//
//---  STRUCTS  ---//
//-----------------//

/// A device is a physical or virtual object that can be controlled or monitored by the system.
#[derive(Debug, Clone, PartialEq)]
pub struct Device<Protocol>
where
    Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
{
    /// Device identifier
    pub name: String,
    /// Description of the device (optional)
    pub description: String,
    /// Instruction set that the device can execute
    pub instructions: HashMap<String, Instruction>,
    /// Protocol which the device can communicate with
    pub protocol: Protocol,
    /// Default arguments for the instructions executed by this device (optional)
    pub default_arguments: HashMap<String, Arguments>,
}

/// Arguments for the instructions
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Arguments {
    /// Argument identifier
    pub name: String,
    /// Argument value
    pub value: Data,
}

//-------------------------//
//---  IMPLEMENTATIONS  ---//
//-------------------------//

impl<Protocol> Device<Protocol>
where
    Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
{
    /// Create a new device
    pub fn new(
        name: String,
        description: String,
        instructions: HashMap<String, Instruction>,
        protocol: Protocol,
        default_arguments: HashMap<String, Arguments>,
    ) -> Self {
        Device {
            name,
            description,
            instructions,
            protocol,
            default_arguments,
        }
    }

    /// Get the instruction with the given name
    pub fn get_instruction(&self, name: &str) -> Option<&Instruction> {
        self.instructions.get(name)
    }

    /// Get the default argument with the given name
    pub fn get_default_argument(&self, name: &str) -> Option<&Arguments> {
        self.default_arguments.get(name)
    }
}
