//----------------//
//---  CRATES  ---//
//----------------//

// Internal modules
use crate::{
    device::{Arguments, Device, Query},
    instruction::Instruction,
    pipeline::{DeviceInstruction, Pipeline, Step, WaitFor},
    Data,
};

// Built-in modules
//// Basic data structures
use std::collections::HashMap;

// External crates
//// Serde: Serialization/Deserialization framework
use serde::de::DeserializeOwned;
//// Tokio: Asynchronous runtime
use tokio::time::{sleep, Duration, Instant};

//------------------------//
//---  IMPLEMENATIONS  ---//
//------------------------//

impl<Protocol> Pipeline<Protocol>
where
    Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
{
    /// Execute the pipeline.
    #[tracing::instrument(name = "Pipeline::execute", skip(self))]
    pub async fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create a parameters stack
        let parameters_stack = HashMap::new();

        // Pipeline data
        tracing::info!("Executing pipeline: {}", self.name);
        let pipeline_length = self.pipeline.len();

        // Iterate over the steps in the pipeline
        for (index, step) in self.pipeline.iter().enumerate() {
            // Execute the step
            tracing::info!("Executing step {}/{}", index + 1, pipeline_length);
            Pipeline::_execute(&self.devices, &step, &parameters_stack).await?;
        }

        Ok(())
    }

    async fn _execute<'a>(
        devices: &'a HashMap<String, Device<Protocol>>,
        step: &Step,
        _parameters_stack: &HashMap<String, Arguments>,
    ) -> Result<Option<HashMap<String, Data>>, Box<dyn std::error::Error>> {
        match step {
            Step::Instruction(device_instruction) => {
                // Get the device
                let device = device_instruction.get_device(devices)?;

                // Get the instruction
                let instruction = device_instruction.get_instruction(device)?;

                // Merge the parameters
                let parameters = device_instruction.merge_parameters(device, _parameters_stack);

                // Render the query
                let query = device_instruction.render_query(device, &parameters)?;

                // Execute the instruction
                let data = device_instruction
                    .execute(device, instruction, &query)
                    .await?;
                Ok(data)
            }
            Step::WaitFor(wait_for) => {
                match &wait_for.metric {
                    // Complex case: wait for a metric to meet a condition
                    Some(metric) => {
                        // Get the device
                        let device = metric.get_device(devices)?;

                        // Get the instruction
                        let instruction = metric.get_instruction(device)?;

                        // Merge the parameters
                        let parameters = metric.merge_parameters(device, _parameters_stack);

                        // Render the query
                        let query = metric.render_query(device, &parameters)?;

                        // Variables used in the loop
                        let mut interval = tokio::time::interval(Duration::from_millis(100)); // TODO: Make this configurable
                        let mut timer: Option<Instant> = None;
                        let mut already_notified = false; // This is to avoid spamming the logs

                        // Loop until the condition is met
                        loop {
                            // Wait for the interval
                            interval.tick().await;
                            // Execute the instruction
                            match metric.execute(device, instruction, &query).await.map_err(
                                |e| {
                                    tracing::error!("Error executing instruction: {}", e);
                                    e
                                },
                            )? {
                                // Check the condition
                                Some(data) => {
                                    if wait_for.check_condition(&data) {
                                        tracing::info!("Condition met. Starting the delay timer.");
                                        // Start the timer
                                        if timer.is_none() {
                                            timer = Some(Instant::now());
                                        }
                                    } else {
                                        if timer.is_some() {
                                            tracing::info!(
                                                "Condition not met. Resetting the delay timer."
                                            );
                                            timer = None;
                                        }
                                    }
                                }
                                // No data returned
                                None => {
                                    if !already_notified {
                                        tracing::error!(
                                            "No data returned. Falling back to the delay time."
                                        );
                                        already_notified = true;
                                    }
                                    // Start the timer
                                    if timer.is_none() {
                                        timer = Some(Instant::now());
                                    }
                                }
                            };
                            // Check if the timer has elapsed
                            if let Some(timer) = timer {
                                if timer.elapsed() >= Duration::from_secs(wait_for.parameters.delay)
                                {
                                    tracing::info!("Delay time elapsed. Continuing the pipeline.");
                                    break;
                                }
                            }
                        }
                        Ok(None)
                    }
                    // Simple case: wait for a fixed amount of time
                    None => {
                        tracing::info!("No metric defined. Using just the delay time");
                        sleep(Duration::from_secs(wait_for.parameters.delay)).await;
                        tracing::info!("Delay time elapsed. Continuing the pipeline.");
                        return Ok(None);
                    }
                }
            }
            Step::Scan(scan) => Ok(None),
        }
    }
}

impl DeviceInstruction {
    #[tracing::instrument(name = "DeviceInstruction::get_device", level = "debug", skip(self))]
    fn get_device<'a, Protocol>(
        &'a self,
        devices: &'a HashMap<String, Device<Protocol>>,
    ) -> Result<&'a Device<Protocol>, Box<dyn std::error::Error>>
    where
        Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
    {
        let device = devices.get(&self.device).ok_or_else(|| {
            tracing::error!("Device not found");
            "Device not found"
        })?;
        tracing::debug!(?device);
        Ok(device)
    }

    #[tracing::instrument(
        name = "DeviceInstruction::merge_parameters",
        level = "debug",
        skip(self, device)
    )]
    fn merge_parameters<Protocol>(
        &self,
        device: &Device<Protocol>,
        parameters_stack: &HashMap<String, Arguments>,
    ) -> HashMap<String, Arguments>
    where
        Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
    {
        // Get the parameters to format the query
        // 1. Get the default arguments from the device
        let mut parameters = device.default_arguments.clone();
        // 2. Add the stack
        parameters.extend(parameters_stack.clone());
        // 3. Add the instruction arguments
        parameters.extend(self.parameters.clone());
        tracing::debug!(?parameters);

        parameters
    }

    #[tracing::instrument(
        name = "DeviceInstruction::get_instruction",
        level = "debug",
        skip(self, device)
    )]
    fn get_instruction<'a, Protocol>(
        &'a self,
        device: &'a Device<Protocol>,
    ) -> Result<&'a Instruction, Box<dyn std::error::Error>>
    where
        Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
    {
        let instruction = device.instructions.get(&self.instruction).ok_or_else(|| {
            tracing::error!("Instruction not found");
            "Instruction not found"
        })?;
        tracing::debug!(?instruction);

        Ok(instruction)
    }

    #[tracing::instrument(
        name = "DeviceInstruction::render_query",
        level = "debug",
        skip(self, device, parameters)
    )]
    fn render_query<Protocol>(
        &self,
        device: &Device<Protocol>,
        parameters: &HashMap<String, Arguments>,
    ) -> Result<String, Box<dyn std::error::Error>>
    where
        Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
    {
        // Get the instruction
        let instruction = self.get_instruction(device)?;

        // Render the query
        let query = instruction.command.render(&parameters)?;
        tracing::debug!(?query);

        Ok(query)
    }

    #[tracing::instrument(
        name = "DeviceInstruction::execute",
        level = "debug",
        skip(self, device, instruction)
    )]
    async fn execute<Protocol>(
        &self,
        device: &Device<Protocol>,
        instruction: &Instruction,
        query: &str,
    ) -> Result<Option<HashMap<String, Data>>, Box<dyn std::error::Error>>
    where
        Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
    {
        // Query the device
        let response = device.protocol.query(&query).await?;
        tracing::debug!(?response);

        // Parse the response
        let data = match response {
            Some(response) => Some(
                instruction
                    .response
                    .as_ref()
                    .ok_or_else(|| {
                        tracing::error!("Instruction was not expecting a response");
                        "Instruction was not expecting a response"
                    })?
                    .parse(&response)?,
            ),
            None => None,
        };
        tracing::debug!(?data);

        Ok(data)
    }
}

impl WaitFor {
    #[tracing::instrument(name = "WaitFor::check_condition", level = "debug")]
    fn check_condition(&self, data: &HashMap<String, Data>) -> bool {
        // Get the value name
        let attribute = match &self.parameters.name {
            Some(name) => name,
            None => {
                tracing::warn!("Parameter name not defined. Skipping the condition check.");
                return true;
            }
        };
        // Get the value
        let value = match data.get(attribute) {
            Some(value) => value,
            None => {
                tracing::error!("Attribute not found. Skipping the condition check.");
                return true;
            }
        };
        // Get teh target value
        let target = match self.parameters.value {
            Some(value) => value,
            None => {
                tracing::error!("Target value not defined. Skipping the condition check.");
                return true;
            }
        };
        // Get the acceptable tolerance/threshold value
        let tolerance = match self.parameters.tolerance {
            Some(tolerance) => tolerance,
            None => {
                tracing::error!("Tolerance not defined. Using 0 as the default value.");
                0.0
            }
        };
        // Check the condition
        match value {
            Data::Integer(value) => {
                if (*value as f64 - target).abs() <= tolerance {
                    tracing::debug!(%value, %target, %tolerance, "Condition met: |{} - {}| <= {}", value, target, tolerance);
                    true
                } else {
                    tracing::debug!(%value, %target, %tolerance, "Condition not met: |{} - {}| > {}", value, target, tolerance);
                    false
                }
            }
            Data::Float(value) => {
                if (value - target).abs() <= tolerance {
                    tracing::debug!(%value, %target, %tolerance, "Condition met: |{} - {}| <= {}", value, target, tolerance);
                    true
                } else {
                    tracing::debug!(%value, %target, %tolerance, "Condition not met: |{} - {}| > {}", value, target, tolerance);
                    false
                }
            }
            _ => {
                tracing::error!("Invalid data type. Skipping the condition check.");
                true
            }
        }
    }
}
