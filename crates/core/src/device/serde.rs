//! Serialization and deserialization for device-related types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A collection of devices.
type Devices = Vec<Device>;

/// During the serialization and deserialization process, the `Protocol` type is used as a key-value pair.
/// Later in the pipeline, the stored data will be converted to one of the implemented communication protocols.
type Protocol = HashMap<String, String>;

/// Represents a device with its properties and associated instructions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Device {
    /// The name of the device.
    pub name: String,
    /// A brief description of the device.
    pub description: Option<String>,
    /// The communication protocol used by the device.
    pub protocol: Protocol,
    /// A list of instructions paths associated with the device.
    pub instructions: Vec<String>,
}
