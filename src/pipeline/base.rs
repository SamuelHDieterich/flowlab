//----------------//
//---  CRATES  ---//
//----------------//

// Internal modules
use crate::{
    device::{Arguments, Device, Query},
    instruction::Instruction,
};

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

#[tracing::instrument]
fn check_pipeline_consistency<Protocol>(
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
                if !_parameters_stack.contains(wait_for_parameter) {
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
            parameters_stack.insert(scan.parameters.variable.clone());

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

// Deserialize the Pipeline struct
impl<'de, Protocol> Deserialize<'de> for Pipeline<Protocol>
where
    Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
{
    fn deserialize<D>(deserializer: D) -> Result<Pipeline<Protocol>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create a new span for the deserialization
        let span = tracing::trace_span!("Pipeline::deserialize");
        let _enter = span.enter();

        // Field identifiers for the Pipeline struct
        // This struct also contain the required extra variables used during the deserialization process
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            Description,
            Instructions, // Instructions defined in the pipeline file (not recommended)
            Devices, // Devices defined in the pipeline file (using 'path' is the recommended approach)
            Pipeline,
        }

        // Pipeline visitor
        struct PipelineVisitor<Protocol> {
            marker: std::marker::PhantomData<Protocol>,
        }
        impl<'de, Protocol> serde::de::Visitor<'de> for PipelineVisitor<Protocol>
        where
            Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
        {
            // Define the type of value that the visitor will return
            type Value = Pipeline<Protocol>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Create a new span for the expecting message
                let span = tracing::trace_span!("PipelineVisitor::expecting");
                let _enter = span.enter();

                formatter.write_str("struct Pipeline")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Pipeline<Protocol>, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                // Create a new span for the visit_map function
                let span = tracing::trace_span!("PipelineVisitor::visit_map");
                let _enter = span.enter();

                // Initialize the fields of the Pipeline struct
                let mut name = String::new();
                let mut description = String::new();
                let mut instructions = HashMap::new(); // Only used during the deserialization process
                let mut devices: HashMap<String, Device<Protocol>> = HashMap::new(); // Only used during the deserialization process
                let mut pipeline = Vec::new();

                // Device can be one of 2 types:
                // 1. A device defined in the pipeline file
                // 2. A device defined in a separate file (recommended approach)
                #[derive(Deserialize, Debug)]
                #[serde(untagged)]
                enum DeviceType<Protocol>
                where
                    Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
                {
                    Path(HashMap<String, PathBuf>),
                    #[serde(bound(deserialize = "Device<Protocol>: Deserialize<'de>"))]
                    Device(Device<Protocol>),
                }

                // Loop through the fields in the map
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            // Create a new span for the Field::Name
                            let span = tracing::trace_span!("Field::Name");
                            let _enter = span.enter();

                            // Check if the name field is duplicated
                            if !name.is_empty() {
                                tracing::error!(
                                    previous_name = %name,
                                    new_name = %map.next_value::<String>()?,
                                    "Field 'name' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("name"));
                            }

                            // Get the name value
                            name = map.next_value()?;
                            tracing::debug!(%name);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Description => {
                            // Create a new span for the Field::Description
                            let span = tracing::trace_span!("Field::Description");
                            let _enter = span.enter();

                            // Check if the description field is duplicated
                            if !description.is_empty() {
                                tracing::error!(
                                    previous_description = %description,
                                    new_description = %map.next_value::<String>()?,
                                    "Field 'description' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("description"));
                            }

                            // Get the description value
                            description = map.next_value()?;
                            tracing::debug!(%description);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Instructions => {
                            // Create a new span for the Field::Instructions
                            let span = tracing::trace_span!("Field::Instructions");
                            let _enter = span.enter();

                            // The instructions parameter in the input file is defined as a list
                            let _instructions: Vec<Instruction> = map.next_value()?;
                            tracing::debug!(instructions_len = %_instructions.len());

                            // Convert the list of instructions to a HashMap
                            for instruction in _instructions {
                                if instructions.contains_key(&instruction.name) {
                                    tracing::warn!(
                                        instruction = %instruction.name,
                                        "Instruction is duplicated"
                                    );
                                }
                                instructions.insert(instruction.name.clone(), instruction);
                            }

                            // Close the span
                            drop(_enter);
                        }
                        Field::Devices => {
                            // Create a new span for the Field::Devices
                            let span = tracing::trace_span!("Field::Devices");
                            let _enter = span.enter();

                            // The devices parameter in the input file is defined as a list
                            let _devices: Vec<DeviceType<Protocol>> = map.next_value()?;
                            tracing::debug!(devices = ?_devices);
                            tracing::debug!(devices_len = %_devices.len());

                            // Convert the list of devices to a HashMap
                            for device in _devices {
                                match device {
                                    // If the device is defined in the pipeline file
                                    DeviceType::Device(device) => {
                                        if devices.contains_key(&device.name) {
                                            tracing::warn!(
                                                device = %device.name,
                                                "Device is duplicated"
                                            );
                                        }
                                        devices.insert(device.name.clone(), device);
                                    }
                                    // If the device is defined in a separate file
                                    DeviceType::Path(path_map) => {
                                        // Get the path to the device file
                                        let path = path_map.get("path").ok_or_else(|| {
                                            tracing::error!("Field 'path' is missing");
                                            serde::de::Error::missing_field("path")
                                        })?;
                                        tracing::debug!(path = %path.display());

                                        // Open the device file
                                        let reader = std::fs::File::open(&path).map_err(|e| {
                                            tracing::error!(
                                                path = %path.display(),
                                                error = ?e,
                                                "Failed to open device file"
                                            );
                                            serde::de::Error::custom(e)
                                        })?;

                                        // Deserialize the device file
                                        // devices:
                                        //   - name: device1
                                        //     ...
                                        let devices_map: HashMap<String, Vec<Device<Protocol>>> =
                                            serde_yaml::from_reader(reader).map_err(|e| {
                                                tracing::error!(
                                                    path = %path.display(),
                                                    error = ?e,
                                                    "Failed to deserialize device file"
                                                );
                                                serde::de::Error::custom(e)
                                            })?;
                                        // Get the list of devices from the device file
                                        let devices_list =
                                            devices_map.get("devices").ok_or_else(|| {
                                                tracing::error!("Field 'devices' is missing");
                                                serde::de::Error::missing_field("devices")
                                            })?;
                                        tracing::debug!(devices_len = %devices_list.len());

                                        // Convert the list of devices to a HashMap
                                        for device in devices_list {
                                            if devices.contains_key(&device.name) {
                                                tracing::warn!(
                                                    device = %device.name,
                                                    "Device is duplicated"
                                                );
                                            }
                                            tracing::debug!(?device);
                                            devices.insert(device.name.clone(), device.clone());
                                        }
                                    }
                                }
                            }

                            // Close the span
                            drop(_enter);
                        }
                        Field::Pipeline => {
                            // Create a new span for the Field::Pipeline
                            let span = tracing::trace_span!("Field::Pipeline");
                            let _enter = span.enter();

                            // Extend the pipeline vector with the steps
                            let _pipeline: Vec<Step> = map.next_value()?;
                            tracing::debug!(pipeline_len = %_pipeline.len());
                            tracing::debug!(?_pipeline);
                            pipeline.extend(_pipeline);

                            // Close the span
                            drop(_enter);
                        }
                    }
                }

                // Check if the mandatory fields are present
                if name.is_empty() {
                    tracing::error!("Field 'name' is missing");
                    return Err(serde::de::Error::missing_field("name"));
                }
                if devices.is_empty() {
                    tracing::error!("Field 'devices' is missing");
                    return Err(serde::de::Error::missing_field("devices"));
                }
                if pipeline.is_empty() {
                    tracing::error!("Field 'pipeline' is missing");
                    return Err(serde::de::Error::missing_field("pipeline"));
                }

                // Verify if the devices and their instructions defined in the pipeline are consistent, i.e., the devices used in the instructions are defined in the devices section and their can perform the asked instructions
                for step in &pipeline {
                    check_pipeline_consistency::<Protocol>(&devices, step).map_err(|e| {
                        tracing::error!(error = %e, "Pipeline consistency check failed");
                        serde::de::Error::custom(e)
                    })?;
                }

                // Return the Pipeline struct
                Ok(Pipeline {
                    name,
                    description,
                    devices,
                    pipeline,
                })
            }
        }
        // Field identifiers for the Pipeline struct
        const FIELDS: &'static [&'static str] =
            &["name", "description", "instructions", "devices", "pipeline"];
        // Deserialize the Pipeline struct
        deserializer.deserialize_struct(
            "Pipeline",
            FIELDS,
            PipelineVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}

// Deserialize the DeviceInstruction struct
impl<'de> Deserialize<'de> for DeviceInstruction {
    fn deserialize<D>(deserializer: D) -> Result<DeviceInstruction, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create a new span for the deserialization
        let span = tracing::trace_span!("DeviceInstruction::deserialize");
        let _enter = span.enter();

        // Field identifiers for the DeviceInstruction struct
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            #[serde(alias = "step")]
            Instruction,
            Device,
            Parameters,
        }

        // DeviceInstruction visitor
        struct DeviceInstructionVisitor;
        impl<'de> serde::de::Visitor<'de> for DeviceInstructionVisitor {
            // Define the type of value that the visitor will return
            type Value = DeviceInstruction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Create a new span for the expecting message
                let span = tracing::trace_span!("DeviceInstructionVisitor::expecting");
                let _enter = span.enter();

                formatter.write_str("struct DeviceInstruction")
            }

            fn visit_map<A>(self, mut map: A) -> Result<DeviceInstruction, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                // Create a new span for the visit_map function
                let span = tracing::trace_span!("DeviceInstructionVisitor::visit_map");
                let _enter = span.enter();

                // Initialize the fields of the DeviceInstruction struct
                let mut instruction = None;
                let mut device = None;
                let mut parameters = HashMap::new();

                // Loop through the fields in the map
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Instruction => {
                            // Create a new span for the Field::Instruction
                            let span = tracing::trace_span!("Field::Instruction");
                            let _enter = span.enter();

                            // Check if the instruction field is duplicated
                            if instruction.is_some() {
                                tracing::error!(
                                    previous_instruction = ?instruction,
                                    new_instruction = ?map.next_value::<String>()?,
                                    "Field 'instruction' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("instruction"));
                            }

                            // Get the instruction value
                            instruction = Some(map.next_value()?);
                            tracing::debug!(?instruction);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Device => {
                            // Create a new span for the Field::Device
                            let span = tracing::trace_span!("Field::Device");
                            let _enter = span.enter();

                            // Check if the device field is duplicated
                            if device.is_some() {
                                tracing::error!(
                                    previous_device = ?device,
                                    new_device = ?map.next_value::<String>()?,
                                    "Field 'device' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("device"));
                            }

                            // Get the device value
                            device = Some(map.next_value()?);
                            tracing::debug!(?device);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Parameters => {
                            // Create a new span for the Field::Parameters
                            let span = tracing::trace_span!("Field::Parameters");
                            let _enter = span.enter();

                            // The parameters parameter in the input file is defined as a list
                            let _parameters: Vec<Arguments> = map.next_value()?;
                            tracing::debug!(parameters_len = %_parameters.len());

                            // Convert the list of parameters to a HashMap
                            for parameter in _parameters {
                                if parameters.contains_key(&parameter.name) {
                                    tracing::warn!(
                                        parameter = %parameter.name,
                                        "Parameter is duplicated"
                                    );
                                }
                                parameters.insert(parameter.name.clone(), parameter);
                            }

                            // Close the span
                            drop(_enter);
                        }
                    }
                }

                // Check if the mandatory fields are present
                let instruction = instruction.ok_or_else(|| {
                    tracing::error!("Field 'instruction' is missing");
                    serde::de::Error::missing_field("instruction")
                })?;
                let device = device.ok_or_else(|| {
                    tracing::error!("Field 'device' is missing");
                    serde::de::Error::missing_field("device")
                })?;

                // Return the DeviceInstruction struct
                Ok(DeviceInstruction {
                    instruction,
                    device,
                    parameters,
                })
            }
        }

        // Field identifiers for the DeviceInstruction struct
        const FIELDS: &'static [&'static str] = &["name", "device", "parameters"];
        // Deserialize the DeviceInstruction struct
        deserializer.deserialize_struct("DeviceInstruction", FIELDS, DeviceInstructionVisitor)
    }
}

// Deserialize the WaitFor struct
impl<'de> Deserialize<'de> for WaitFor {
    fn deserialize<D>(deserializer: D) -> Result<WaitFor, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create a new span for the deserialization
        let span = tracing::trace_span!("WaitFor::deserialize");
        let _enter = span.enter();

        // Field identifiers for the WaitFor struct
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            #[serde(rename = "step")]
            Name,
            Metric,
            Parameters,
        }

        // WaitFor visitor
        struct WaitForVisitor;
        impl<'de> serde::de::Visitor<'de> for WaitForVisitor {
            // Define the type of value that the visitor will return
            type Value = WaitFor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Create a new span for the expecting message
                let span = tracing::trace_span!("WaitForVisitor::expecting");
                let _enter = span.enter();

                formatter.write_str("struct WaitFor")
            }

            fn visit_map<A>(self, mut map: A) -> Result<WaitFor, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                // Create a new span for the visit_map function
                let span = tracing::trace_span!("WaitForVisitor::visit_map");
                let _enter = span.enter();

                // Initialize the fields of the WaitFor struct
                let mut metric = None;
                let mut parameters = WaitForParameters {
                    name: None,
                    value: None,
                    tolerance: None,
                    delay: 0,
                };

                // Loop through the fields in the map
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {} // The name field is always "Wait for", we can ignore it
                        Field::Metric => {
                            // Create a new span for the Field::Metric
                            let span = tracing::trace_span!("Field::Metric");
                            let _enter = span.enter();

                            tracing::info!("Field 'metric' is optional");

                            // Check if the metric field is duplicated
                            if metric.is_some() {
                                tracing::error!(
                                    previous_metric = ?metric,
                                    new_metric = ?map.next_value::<DeviceInstruction>()?,
                                    "Field 'metric' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("metric"));
                            }

                            // Get the metric value
                            metric = Some(map.next_value()?);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Parameters => {
                            // Create a new span for the Field::Parameters
                            let span = tracing::trace_span!("Field::Parameters");
                            let _enter = span.enter();

                            // Get the parameters value
                            parameters = map.next_value()?;
                            tracing::debug!(?parameters);

                            // Close the span
                            drop(_enter);
                        }
                    }
                }

                // Check if the mandatory fields are present
                //// Since the name field is always "Wait for", we can ignore it
                //// Since the metric field is optional, we can ignore it
                //// Since the parameters minimum requirements is the delay that has a default value, we can ignore it

                // Return the WaitFor struct
                Ok(WaitFor { metric, parameters })
            }
        }

        // Field identifiers for the WaitFor struct
        const FIELDS: &'static [&'static str] = &["name", "metric", "parameters"];
        // Deserialize the WaitFor struct
        deserializer.deserialize_struct("WaitFor", FIELDS, WaitForVisitor)
    }
}

// Deserialize the WaitForParameters struct
impl<'de> Deserialize<'de> for WaitForParameters {
    fn deserialize<D>(deserializer: D) -> Result<WaitForParameters, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create a new span for the deserialization
        let span = tracing::trace_span!("WaitForParameters::deserialize");
        let _enter = span.enter();

        // Field identifiers for the WaitForParameters struct
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            Value,
            Tolerance,
            Delay,
        }

        // WaitForParameters visitor
        struct WaitForParametersVisitor;
        impl<'de> serde::de::Visitor<'de> for WaitForParametersVisitor {
            // Define the type of value that the visitor will return
            type Value = WaitForParameters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Create a new span for the expecting message
                let span = tracing::trace_span!("WaitForParametersVisitor::expecting");
                let _enter = span.enter();

                formatter.write_str("struct WaitForParameters")
            }

            fn visit_map<A>(self, mut map: A) -> Result<WaitForParameters, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                // Create a new span for the visit_map function
                let span = tracing::trace_span!("WaitForParametersVisitor::visit_map");
                let _enter = span.enter();

                // Initialize the fields of the WaitForParameters struct
                let mut name = None;
                let mut value = None;
                let mut tolerance = None;
                let mut delay = 0;

                // Loop through the fields in the map
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            // Create a new span for the Field::Name
                            let span = tracing::trace_span!("Field::Name");
                            let _enter = span.enter();

                            // Check if the name field is duplicated
                            if name.is_some() {
                                tracing::error!(
                                    previous_name = ?name,
                                    new_name = ?map.next_value::<String>()?,
                                    "Field 'name' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("name"));
                            }

                            // Get the name value
                            name = Some(map.next_value()?);
                            tracing::debug!(?name);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Value => {
                            // Create a new span for the Field::Value
                            let span = tracing::trace_span!("Field::Value");
                            let _enter = span.enter();

                            // Check if the value field is duplicated
                            if value.is_some() {
                                tracing::error!(
                                    previous_value = ?value,
                                    new_value = ?map.next_value::<f32>()?,
                                    "Field 'value' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("value"));
                            }

                            // Get the value value
                            value = Some(map.next_value()?);
                            tracing::debug!(?value);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Tolerance => {
                            // Create a new span for the Field::Tolerance
                            let span = tracing::trace_span!("Field::Tolerance");
                            let _enter = span.enter();

                            // Check if the tolerance field is duplicated
                            if tolerance.is_some() {
                                tracing::error!(
                                    previous_tolerance = ?tolerance,
                                    new_tolerance = ?map.next_value::<f32>()?,
                                    "Field 'tolerance' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("tolerance"));
                            }

                            // Get the tolerance value
                            tolerance = Some(map.next_value()?);
                            tracing::debug!(?tolerance);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Delay => {
                            // Create a new span for the Field::Delay
                            let span = tracing::trace_span!("Field::Delay");
                            let _enter = span.enter();

                            // Check if the delay field is duplicated
                            if delay != 0 {
                                tracing::error!(
                                    previous_delay = %delay,
                                    new_delay = %map.next_value::<u32>()?,
                                    "Field 'delay' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("delay"));
                            }

                            // Get the delay value
                            delay = map.next_value()?;
                            tracing::debug!(delay);

                            // Close the span
                            drop(_enter);
                        }
                    }
                }

                // Check if the mandatory fields are present
                //// Since the name field is optional, we can ignore it
                //// Since the value field is optional, we can ignore it
                //// Since the tolerance field is optional, we can ignore it
                //// Since the delay field has a default value, we can ignore it

                // Return the WaitForParameters struct
                Ok(WaitForParameters {
                    name,
                    value,
                    tolerance,
                    delay,
                })
            }
        }

        // Field identifiers for the WaitForParameters struct
        const FIELDS: &'static [&'static str] = &["name", "value", "tolerance", "delay"];
        // Deserialize the WaitForParameters struct
        deserializer.deserialize_struct("WaitForParameters", FIELDS, WaitForParametersVisitor)
    }
}

// Deserialize the Scan struct
impl<'de> Deserialize<'de> for Scan {
    fn deserialize<D>(deserializer: D) -> Result<Scan, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create a new span for the deserialization
        let span = tracing::trace_span!("Scan::deserialize");
        let _enter = span.enter();

        // Field identifiers for the Scan struct
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            #[serde(rename = "step")]
            Name,
            Metrics,
            Parameters,
            Datafile,
            Measures,
        }

        // Scan visitor
        struct ScanVisitor;
        impl<'de> serde::de::Visitor<'de> for ScanVisitor {
            // Define the type of value that the visitor will return
            type Value = Scan;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Create a new span for the expecting message
                let span = tracing::trace_span!("ScanVisitor::expecting");
                let _enter = span.enter();

                formatter.write_str("struct Scan")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Scan, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                // Create a new span for the visit_map function
                let span = tracing::trace_span!("ScanVisitor::visit_map");
                let _enter = span.enter();

                // Initialize the fields of the Scan struct
                let mut metrics = Vec::new();
                let mut parameters = None;
                let mut datafile = None;
                let mut measures = Vec::new();

                // Loop through the fields in the map
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {} // The name field is always "Scan", we can ignore it
                        Field::Metrics => {
                            // Create a new span for the Field::Metrics
                            let span = tracing::trace_span!("Field::Metrics");
                            let _enter = span.enter();

                            // Get the metrics value
                            let metrics_value = map.next_value::<Vec<Step>>()?;
                            metrics.extend(metrics_value);
                            tracing::debug!(?metrics);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Parameters => {
                            // Create a new span for the Field::Parameters
                            let span = tracing::trace_span!("Field::Parameters");
                            let _enter = span.enter();

                            // Check if the parameters field is duplicated
                            if parameters.is_some() {
                                tracing::error!(
                                    previous_parameters = ?parameters,
                                    new_parameters = ?map.next_value::<ScanParameters>()?,
                                    "Field 'parameters' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }

                            // Get the parameters value
                            parameters = Some(map.next_value()?);
                            tracing::debug!(?parameters);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Datafile => {
                            // Create a new span for the Field::Datafile
                            let span = tracing::trace_span!("Field::Datafile");
                            let _enter = span.enter();

                            // Check if the datafile field is duplicated
                            if datafile.is_some() {
                                tracing::error!(
                                    previous_datafile = ?datafile,
                                    new_datafile = ?map.next_value::<PathBuf>()?,
                                    "Field 'datafile' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("datafile"));
                            }

                            // Get the datafile value
                            datafile = Some(map.next_value()?);
                            tracing::debug!(?datafile);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Measures => {
                            // Create a new span for the Field::Measures
                            let span = tracing::trace_span!("Field::Measures");
                            let _enter = span.enter();

                            // The measures parameter in the input file is defined as a list
                            let _measures: Vec<Step> = map.next_value()?;
                            tracing::debug!(measures_len = %_measures.len());

                            // Convert the list of measures to a Vec
                            measures = _measures;
                            tracing::debug!(?measures);

                            // Close the span
                            drop(_enter);
                        }
                    }
                }

                // Check if the mandatory fields are present
                //// Since the name is always "Scan", we can ignore it
                if metrics.is_empty() {
                    tracing::error!("Field 'metrics' is missing");
                    return Err(serde::de::Error::missing_field("metrics"));
                }
                let parameters = parameters.ok_or_else(|| {
                    tracing::error!("Field 'parameters' is missing");
                    serde::de::Error::missing_field("parameters")
                })?;
                if measures.is_empty() {
                    tracing::error!("Field 'measures' is missing");
                    return Err(serde::de::Error::missing_field("measures"));
                }

                // Return the Scan struct
                Ok(Scan {
                    metrics,
                    parameters,
                    datafile,
                    measures,
                })
            }
        }

        // Field identifiers for the Scan struct
        const FIELDS: &'static [&'static str] = &[
            "name",
            "metrics",
            "scan_type",
            "parameters",
            "datafile",
            "measures",
        ];
        // Deserialize the Scan struct
        deserializer.deserialize_struct("Scan", FIELDS, ScanVisitor)
    }
}

// Deserialize the ScanParameters struct
impl<'de> Deserialize<'de> for ScanParameters {
    fn deserialize<D>(deserializer: D) -> Result<ScanParameters, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create a new span for the deserialization
        let span = tracing::trace_span!("ScanParameters::deserialize");
        let _enter = span.enter();

        // Field identifiers for the ScanParameters struct
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Variable,
            Start,
            Stop,
            Step,
        }

        // ScanParameters visitor
        struct ScanParametersVisitor;
        impl<'de> serde::de::Visitor<'de> for ScanParametersVisitor {
            // Define the type of value that the visitor will return
            type Value = ScanParameters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Create a new span for the expecting message
                let span = tracing::trace_span!("ScanParametersVisitor::expecting");
                let _enter = span.enter();

                formatter.write_str("struct ScanParameters")
            }

            fn visit_map<A>(self, mut map: A) -> Result<ScanParameters, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                // Create a new span for the visit_map function
                let span = tracing::trace_span!("ScanParametersVisitor::visit_map");
                let _enter = span.enter();

                // Initialize the fields of the ScanParameters struct
                let mut variable = String::new();
                let mut start = None;
                let mut stop = None;
                let mut step = None;

                // Loop through the fields in the map
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Variable => {
                            // Create a new span for the Field::Variable
                            let span = tracing::trace_span!("Field::Variable");
                            let _enter = span.enter();

                            // Check if the variable field is duplicated
                            if !variable.is_empty() {
                                tracing::error!(
                                    previous_variable = %variable,
                                    new_variable = %map.next_value::<String>()?,
                                    "Field 'variable' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("variable"));
                            }

                            // Get the variable value
                            variable = map.next_value()?;
                            tracing::debug!(%variable);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Start => {
                            // Create a new span for the Field::Start
                            let span = tracing::trace_span!("Field::Start");
                            let _enter = span.enter();

                            // Check if the start field is duplicated
                            if start.is_some() {
                                tracing::error!(
                                    previous_start = ?start,
                                    new_start = ?map.next_value::<f64>()?,
                                    "Field 'start' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("start"));
                            }

                            // Get the start value
                            start = Some(map.next_value()?);
                            tracing::debug!(?start);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Stop => {
                            // Create a new span for the Field::Stop
                            let span = tracing::trace_span!("Field::Stop");
                            let _enter = span.enter();

                            // Check if the stop field is duplicated
                            if stop.is_some() {
                                tracing::error!(
                                    previous_stop = ?stop,
                                    new_stop = ?map.next_value::<f64>()?,
                                    "Field 'stop' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("stop"));
                            }

                            // Get the stop value
                            stop = Some(map.next_value()?);
                            tracing::debug!(?stop);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Step => {
                            // Create a new span for the Field::Step
                            let span = tracing::trace_span!("Field::Step");
                            let _enter = span.enter();

                            // Check if the step field is duplicated
                            if step.is_some() {
                                tracing::error!(
                                    previous_step = ?step,
                                    new_step = ?map.next_value::<f64>()?,
                                    "Field 'step' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("step"));
                            }

                            // Get the step value
                            step = Some(map.next_value()?);
                            tracing::debug!(?step);

                            // Close the span
                            drop(_enter);
                        }
                    }
                }

                // Check if the mandatory fields are present
                if variable.is_empty() {
                    tracing::error!("Field 'variable' is missing");
                    return Err(serde::de::Error::missing_field("variable"));
                }
                let start = start.ok_or_else(|| {
                    tracing::error!("Field 'start' is missing");
                    serde::de::Error::missing_field("start")
                })?;
                let stop = stop.ok_or_else(|| {
                    tracing::error!("Field 'stop' is missing");
                    serde::de::Error::missing_field("stop")
                })?;
                let step = step.ok_or_else(|| {
                    tracing::error!("Field 'step' is missing");
                    serde::de::Error::missing_field("step")
                })?;

                // Check if other certain conditions are met
                if step == 0.0 {
                    tracing::error!("Field 'step' is zero");
                    return Err(serde::de::Error::custom("Field 'step' is zero"));
                }
                if start == stop {
                    tracing::error!("Field 'start' is equal to 'stop'");
                    return Err(serde::de::Error::custom("Field 'start' is equal to 'stop'"));
                }

                // Return the ScanParameters struct
                Ok(ScanParameters {
                    variable,
                    start,
                    stop,
                    step,
                })
            }
        }

        // Field identifiers for the ScanParameters struct
        const FIELDS: &'static [&'static str] = &["variable", "start", "stop", "step"];
        // Deserialize the ScanParameters struct
        deserializer.deserialize_struct("ScanParameters", FIELDS, ScanParametersVisitor)
    }
}
