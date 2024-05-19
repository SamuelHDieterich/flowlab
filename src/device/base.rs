//! # Base
//! Base building blocks for creating devices with generic protocols.
//!
//! With this module, you can create `Device`s with any `Protocol` that implements the `Query` trait.
//! - `Device`: Physical or virtual object that can be controlled or monitored by the system.
//! - `Protocol`: Communication protocol that the device can communicate with, for instance, `TCP` and `Serial`.
//! - `Query`: Trait that allows the device to send commands and receive responses.

// Allows traits to have async functions
// This is required for the Query trait for Rust version 1.75 or below and, for now, it is recommended for public traits.
use async_trait::async_trait;

// Serde: Serialization/Deserialization framework
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A device is a physical or virtual object that can be controlled or monitored by the system.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Device<Protocol> {
    /// Name of the device
    pub name: String,
    /// Instructions set that the device can execute
    pub instruction: Vec<String>,
    /// Protocol which the device can communicate with
    pub protocol: Protocol,
    /// Default arguments for the instructions
    pub default_arguments: Option<Vec<Arguments>>,
}

/// Display implementation for the Device struct
///
/// name: <name>
/// instruction: <instruction>
/// protocol: <protocol>
/// default_arguments: <default_arguments>
impl<Protocol> std::fmt::Display for Device<Protocol> 
where
    Protocol: std::fmt::Display + Serialize
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let serialized = match serde_yaml::to_string(self) {
            Ok(s) => s,
            Err(_) => return Err(std::fmt::Error),
        };
        write!(f, "{}", serialized)
    }
}

/// Arguments for the instructions
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Arguments {
    pub name: String,
    pub value: String,
}

impl std::fmt::Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let serialized = match serde_yaml::to_string(self) {
            Ok(s) => s,
            Err(_) => return Err(std::fmt::Error),
        };
        write!(f, "{}", serialized)
    }
}

/// A protocol must implement the Query trait which allows the device to send commands and receive responses.
#[async_trait]
pub trait Query {
    /// Send a command to the device and return the response (if any).
    async fn query(&self, command: &str) -> Result<Option<String>, std::io::Error>;
}
