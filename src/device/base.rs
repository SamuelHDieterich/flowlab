//! # Base
//! Base building blocks for creating devices with generic protocols.
//!
//! With this module, you can create `Device`s with any `Protocol` that implements the `Query` trait.
//! - `Device`: Physical or virtual object that can be controlled or monitored by the system.
//! - `Protocol`: Communication protocol that the device can communicate with, for instance, `TCP` and `Serial`.
//! - `Query`: Trait that allows the device to send commands and receive responses.

use crate::{from_map_to_vec, from_vec_to_map, Data, DataType, MapKey, Mapify};
use flowlab_macros::{MapKey, Mapify};
use std::{collections::HashMap, path::PathBuf};

// Allows traits to have async functions
// This is required for the Query trait for Rust version 1.75 or below and, for now, it is recommended for public traits.
use async_trait::async_trait;

// Serde: Serialization/Deserialization framework
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Mapify)]
pub struct DeviceCollection<Protocol>(HashMap<PathBuf, Vec<Device<Protocol>>>);

impl<Protocol> DeviceCollection<Protocol> {
    pub fn new(filepath: PathBuf, devices: Vec<Device<Protocol>>) -> Self {
        let mut map = HashMap::new();
        map.insert(filepath, devices);
        Self(map)
    }
}

impl<Protocol> Default for DeviceCollection<Protocol> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}
/// A device is a physical or virtual object that can be controlled or monitored by the system.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Device<Protocol> {
    /// Name of the device
    pub name: String,
    /// Instructions set that the device can execute
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub instruction: Vec<String>,
    /// Protocol which the device can communicate with
    pub protocol: Protocol,
    /// Default arguments for the instructions
    #[serde(
        serialize_with = "from_map_to_vec",
        deserialize_with = "from_vec_to_map",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub default_arguments: HashMap<String, Arguments>,
}

/// Arguments for the instructions
#[derive(Debug, Serialize, Deserialize, PartialEq, MapKey)]
pub struct Arguments {
    pub name: String,
    pub value: Data,
}

pub fn find_device_with_name<'a, Protocol>(
    devices: &'a Vec<Device<Protocol>>,
    name: &str,
) -> Option<&'a Device<Protocol>> {
    devices.iter().find(|d| d.name == name)
}

/// A protocol must implement the Query trait which allows the device to send commands and receive responses.
#[async_trait]
pub trait Query {
    /// Send a command to the device and return the response (if any).
    async fn query(&self, command: &str) -> Result<Option<String>, std::io::Error>;
}
