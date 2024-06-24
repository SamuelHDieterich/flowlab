//----------------//
//---  CRATES  ---//
//----------------//

// Internal modules
use crate::device::{Arguments, Device, Query};

// Built-in modules
//// Basic data structures
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

// External crates
//// Serde: Serialization/Deserialization framework
use serde::{de::DeserializeOwned, Deserialize};

//-----------------//
//---  STRUCTS  ---//
//-----------------//

/// The `Pipeline` struct is the main structure that defines the pipeline configuration.
#[derive(Debug, Clone)]
pub struct Pipeline<Protocol>
where
    Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
{
    pub name: String,
    pub description: String,
    pub devices: HashMap<String, Device<Protocol>>,
    pub pipeline: Vec<Step>,
}

/// The `Step` enum is used to define the different types of instructions that can be performed in a pipeline, including commands defined by the device as well as generic instructions (such as waiting for a device to reach a certain state).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Step {
    Instruction(DeviceInstruction),
    WaitFor(WaitFor),
    Scan(Scan),
}

/// The `DeviceInstruction` struct is used to define the instructions that a device can perform.
#[derive(Debug, Clone)]
pub struct DeviceInstruction {
    pub instruction: String,
    pub device: String,
    pub parameters: HashMap<String, Arguments>,
}

/// The `WaitFor` struct is a generic instruction that can be used to wait for a device to reach a certain state.
#[derive(Debug, Clone)]
pub struct WaitFor {
    pub metric: Option<DeviceInstruction>, // If None, fallback to a simple timer
    pub parameters: WaitForParameters,
}

/// The `WaitForParameters` struct is used to define the parameters that a WaitFor instruction takes.
#[derive(Debug, Clone)]
pub struct WaitForParameters {
    pub name: Option<String>,
    pub value: Option<f32>,
    pub tolerance: Option<f32>,
    pub delay: u32,
}

/// The `Scan` struct is a generic instruction that can be used to scan/loop a specific property to perform some measurements.
#[derive(Debug, Clone)]
pub struct Scan {
    pub metrics: Vec<Step>,
    // pub scan_type: ScanType,
    pub parameters: ScanParameters,
    pub datafile: Option<PathBuf>,
    pub measures: Vec<Step>,
}

/// The `ScanParameters` struct is used to define the parameters that a Scan instruction takes.
#[derive(Debug, Clone)]
pub struct ScanParameters {
    pub variable: String,
    pub start: f64,
    pub stop: f64,
    pub step: f64,
}

//-------------------//
//---  FUNCTIONS  ---//
//-------------------//

#[tracing::instrument(level = "debug")]
pub fn check_pipeline_consistency<Protocol>(
    devices: &HashMap<String, Device<Protocol>>,
    pipeline: &Step,
) -> Result<(), String>
where
    Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
{
    _check_pipeline_consistency::<Protocol>(devices, pipeline, &HashSet::new())
}

fn _check_pipeline_consistency<Protocol>(
    devices: &HashMap<String, Device<Protocol>>,
    pipeline: &Step,
    _parameters_stack: &HashSet<String>,
) -> Result<(), String>
where
    Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
{
    match pipeline {
        Step::Instruction(instruction) => {
            // Check if the device is defined in the devices section
            let device = devices.get(&instruction.device).ok_or_else(|| {
                tracing::error!(
                    device = %instruction.device,
                    "Device is not defined in the devices section"
                );
                format!(
                    "Device '{}' is not defined in the devices section",
                    instruction.device
                )
            })?;

            // Check if the device can perform the instruction
            if !device.instructions.contains_key(&instruction.instruction) {
                tracing::error!(
                    device = %instruction.device,
                    instruction = %instruction.instruction,
                    "Device cannot perform the instruction"
                );
                return Err(format!(
                    "Device '{}' cannot perform the instruction '{}'",
                    instruction.device, instruction.instruction
                ));
            }

            // Get all available parameters in the scope
            //// Stack (example: Scan variable)
            let mut available_parameters = _parameters_stack.clone();
            //// Device default parameters
            available_parameters.extend(device.default_arguments.keys().cloned());
            //// Instruction defined parameters
            available_parameters.extend(instruction.parameters.keys().cloned());

            // Check if there are any missing parameters
            for parameter in device.instructions[&instruction.instruction]
                .command
                .parameters
                .keys()
            {
                if !available_parameters.contains(parameter) {
                    tracing::error!(
                        device = %instruction.device,
                        instruction = %instruction.instruction,
                        parameter = %parameter,
                        "Missing parameter"
                    );
                    return Err(format!(
                        "Missing parameter '{}' for the instruction '{}'",
                        parameter, instruction.instruction
                    ));
                }
            }
        }
        // We can recall the function recursively for the other types of steps
        Step::WaitFor(wait_for) => {
            // Check if the metric is properly defined
            if let Some(metric) = &wait_for.metric {
                _check_pipeline_consistency::<Protocol>(
                    devices,
                    &Step::Instruction(metric.clone()),
                    _parameters_stack,
                )?;
            }
            // Check if the metric response has a parameter that corresponds to the wait for parameter
            if let Some(wait_for_parameter) = &wait_for.parameters.name {
                // Include the metric response values to the stack
                let mut parameters_stack = _parameters_stack.clone();
                if let Some(metric) = &wait_for.metric {
                    if let Some(metric_instruction) = devices[&metric.device]
                        .instructions
                        .get(&metric.instruction)
                    {
                        let metric_response = metric_instruction
                            .response
                            .as_ref()
                            .ok_or_else(|| {
                                tracing::error!(
                                    device = %metric.device,
                                    instruction = %metric.instruction,
                                    "Instruction does not have a response"
                                );
                                format!(
                                    "Instruction '{}' of device '{}' does not have a response",
                                    metric.instruction, metric.device
                                )
                            })?
                            .parameters
                            .keys()
                            .cloned()
                            .collect::<Vec<String>>();
                        parameters_stack.extend(metric_response);
                    }
                }
                tracing::debug!(?wait_for_parameter);
                tracing::debug!(?parameters_stack);
                if !parameters_stack.contains(wait_for_parameter) {
                    tracing::error!(
                        parameter = %wait_for_parameter,
                        "Parameter is missing in the scope"
                    );
                    return Err(format!(
                        "Parameter '{}' is missing in the scope",
                        wait_for_parameter
                    ));
                }
            }
        }
        Step::Scan(scan) => {
            // Add the scan variable to the stack
            let mut parameters_stack = _parameters_stack.clone();
            let scan_variable = scan.parameters.variable.clone();
            tracing::debug!(?scan_variable);
            parameters_stack.insert(scan_variable);

            // Check if the scan metrics are consistent
            for metric in &scan.metrics {
                _check_pipeline_consistency::<Protocol>(devices, metric, &parameters_stack)?;
            }
            // Check if the scan measures are consistent
            for measure in &scan.measures {
                _check_pipeline_consistency::<Protocol>(devices, measure, &parameters_stack)?;
            }
        }
    }
    Ok(())
}

//------------------------//
//---  IMPLEMENATIONS  ---//
//------------------------//

impl<Protocol> Pipeline<Protocol>
where
    Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
{
    /// Create a new pipeline configuration
    pub fn new(
        name: String,
        description: String,
        devices: HashMap<String, Device<Protocol>>,
        pipeline: Vec<Step>,
    ) -> Self {
        Self {
            name,
            description,
            devices,
            pipeline,
        }
    }
}

impl DeviceInstruction {
    /// Create a new DeviceInstruction struct
    pub fn new(
        instruction: String,
        device: String,
        parameters: HashMap<String, Arguments>,
    ) -> Self {
        Self {
            instruction,
            device,
            parameters,
        }
    }
}
