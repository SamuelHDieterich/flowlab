/*
 ____
| __ )  __ _ ___  ___
|  _ \ / _` / __|/ _ \
| |_) | (_| \__ \  __/
|____/ \__,_|___/\___|

Base building blocks for creating devices with generic protocols.

*/

// Allows traits to have async functions
use async_trait::async_trait;

// Serde: Serialization/Deserialization framework
use serde::{Deserialize, Serialize};

/// A device is a physical or virtual object that can be controlled or monitored by the program
#[derive(Debug, Serialize, Deserialize)]
pub struct Device<Protocol> {
    /// Name of the device
    pub name: String,
    /// Instructions set that the device can execute
    pub instruction: Vec<String>,
    /// Protocol which the device can communicate with
    pub protocol: Protocol,
}

/// A protocol must implement the Query trait
/// This trait allows the device to send commands and receive responses
#[async_trait]
pub trait Query {
    /// Send a command to the device and return the response (if any).
    async fn query(&self, command: &str) -> Result<Option<String>, std::io::Error>;
}
