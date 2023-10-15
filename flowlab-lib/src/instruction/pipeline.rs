/*
 ____  _            _ _
|  _ \(_)_ __   ___| (_)_ __   ___
| |_) | | '_ \ / _ \ | | '_ \ / _ \
|  __/| | |_) |  __/ | | | | |  __/
|_|   |_| .__/ \___|_|_|_| |_|\___|
        |_|

The pipeline module contains the code responsible to parse the pipeline instructions.

*/

// Enum of possible data types
use super::base::DataType;

// Serde: Serialization/Deserialization framework
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// Filepath
use std::path::PathBuf;

/// The PipelineStep enum is used to define the different types of instructions that can be performed in a pipeline, including commands defined by the device as well as generic instructions (such as waiting for a device to reach a certain state).
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PipelineStep {
    DeviceInstruction(DeviceInstruction),
    WaitFor(WaitFor),
    Scan(Scan),
}

/// The DeviceInstruction struct is used to define the instructions that a device can perform.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DeviceInstruction {
    #[serde(rename = "instruction")]
    pub name: String,
    pub device: String,
    pub parameters: Option<Vec<DeviceInstructionParameters>>,
}

/// The DeviceInstructionParameters struct is used to define the parameters that a DeviceInstruction instruction takes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DeviceInstructionParameters {
    pub name: String,
    pub value: DataType, // Maybe change to a generic type? (e.g. String, f32, u32, etc.)
}

/// The WaitFor struct is a generic instruction that can be used to wait for a device to reach a certain state.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WaitFor {
    #[serde(rename = "instruction")]
    pub name: String, // Always "Wait for", so can be ignored
    pub metric: DeviceInstruction,
    pub parameters: WaitForParameters,
}

/// The WaitForParameters struct is used to define the parameters that a WaitFor instruction takes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WaitForParameters {
    pub value: f32,
    pub tolerance: f32,
    pub delay: u32,
}

/// The Scan struct is a generic instruction that can be used to scan/loop a specific property to perform some measurements.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ScanType {
    Settle,
    Sweap,
}

/// The ScanParameters struct is used to define the parameters that a Scan instruction takes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ScanParameters {
    pub variable: String,
    pub start: Number,
    pub stop: Number,
    pub step: Number,
}

/// Enum with only numbers (integer or float)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Number {
    Integer(i32),
    Float(f32),
}
