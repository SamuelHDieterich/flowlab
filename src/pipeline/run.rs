//----------------//
//---  CRATES  ---//
//----------------//

// Internal modules
use crate::{
    device::{Arguments, Device, Query},
    pipeline::{DeviceInstruction, Pipeline, Scan, Step, WaitFor},
    Data,
};

// Built-in modules
//// Basic data structures
use std::{collections::HashMap, ops::Deref};

// External crates
//// Serde: Serialization/Deserialization framework
use serde::de::DeserializeOwned;

//------------------------//
//---  IMPLEMENATIONS  ---//
//------------------------//

impl<Protocol> Pipeline<Protocol>
where
    Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
{
    /// Execute the pipeline.
    #[tracing::instrument]
    pub async fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create a parameters stack
        let parameters_stack = HashMap::new();

        // Iterate over the steps in the pipeline
        for step in &self.pipeline {
            // Execute the step
            Pipeline::_execute(&self.devices, &step, &parameters_stack).await?;
        }

        Ok(())
    }

    #[tracing::instrument(level = "debug")]
    async fn _execute(
        devices: &HashMap<String, Device<Protocol>>,
        step: &Step,
        _parameters_stack: &HashMap<String, Arguments>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match step {
            Step::Instruction(instruction) => {}
            Step::WaitFor(wait_for) => {}
            Step::Scan(scan) => {}
        }
        Ok(())
    }
}

impl DeviceInstruction {
    #[tracing::instrument(level = "debug")]
    async fn execute<Protocol>(
        &self,
        device: &Device<Protocol>,
        parameters_stack: &HashMap<String, Arguments>,
    ) -> Result<Option<HashMap<String, Data>>, Box<dyn std::error::Error>>
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

        // Get the instruction
        let instruction = device.instructions.get(&self.instruction).ok_or_else(|| {
            tracing::error!("Instruction not found");
            "Instruction not found"
        })?;

        // Render the query
        let query = instruction.command.render(&parameters)?;
        tracing::debug!(?query);

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
