// Enum of possible data types
use crate::{from_map_to_vec, from_vec_to_map, Data, MapKey};

use flowlab_macros::MapKey;
// Serde: Serialization/Deserialization framework
use serde::{Deserialize, Serialize};

// Filepath
use std::{collections::HashMap, path::PathBuf};

/// The `PipelineStep` enum is used to define the different types of instructions that can be performed in a pipeline, including commands defined by the device as well as generic instructions (such as waiting for a device to reach a certain state).
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PipelineStep {
    DeviceInstruction(DeviceInstruction),
    WaitFor(WaitFor),
    Scan(Scan),
}

/// The `DeviceInstruction` struct is used to define the instructions that a device can perform.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInstruction {
    #[serde(rename = "instruction")]
    pub name: String,
    pub device: String,
    #[serde(
        serialize_with = "from_map_to_vec",
        deserialize_with = "from_vec_to_map",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub parameters: HashMap<String, DeviceInstructionParameters>,
}

/// The `DeviceInstructionParameters` struct is used to define the parameters that a DeviceInstruction instruction takes.
#[derive(Debug, PartialEq, MapKey, Serialize, Deserialize)]
pub struct DeviceInstructionParameters {
    pub name: String,
    pub value: Data, // Maybe change to a generic type? (e.g. String, f32, u32, etc.)
}

/// The `WaitFor` struct is a generic instruction that can be used to wait for a device to reach a certain state.
#[derive(Debug, Serialize, Deserialize)]
pub struct WaitFor {
    #[serde(rename = "instruction")]
    pub name: String, // Always "Wait for", so can be ignored
    pub metric: Option<DeviceInstruction>,
    pub parameters: WaitForParameters,
}

/// The `WaitForParameters` struct is used to define the parameters that a WaitFor instruction takes.
#[derive(Debug, Serialize, Deserialize)]
pub struct WaitForParameters {
    pub name: Option<String>, // TODO: If None, metric should also be None.
    pub value: f32,
    pub tolerance: f32,
    pub delay: u32,
}

/// The `Scan` struct is a generic instruction that can be used to scan/loop a specific property to perform some measurements.
#[derive(Debug, Serialize, Deserialize)]
pub struct Scan {
    #[serde(rename = "instruction")]
    pub name: String, // Always "Scan", so can be ignored
    pub metric: DeviceInstruction,
    #[serde(rename = "type")]
    pub scan_type: ScanType,
    pub parameters: ScanParameters,
    pub datafile: PathBuf,
    pub measures: Vec<DeviceInstruction>,
}

/// `Scan` can be one of two types: `Settle` or `Sweap`. `Settle` means each record will be
/// performed only when the scan step is completed. `Sweap` means the step is non-blocking and each
/// registry will be performed as soon as possible.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScanType {
    Settle,
    Sweap,
}

/// The `ScanParameters` struct is used to define the parameters that a Scan instruction takes.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScanParameters {
    pub variable: String,
    pub start: Number,
    pub stop: Number,
    pub step: Number,
}

/// Enum with only numbers (integer or float)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Number {
    Integer(i64),
    Float(f64),
}
