//-----------------//
//---  MODULES  ---//
//-----------------//

// Internal modules
use super::base::*;
use super::Query;
use crate::{instruction::Instruction, Data};

use std::collections::BTreeMap;
// Built-in modules
//// Basic data structures
use std::{collections::HashMap, path::PathBuf};

// External crates
//// Serde: Serialization/Deserialization framework
use serde::{de::DeserializeOwned, Deserialize};

//-------------------------//
//---  IMPLEMENTATIONS  ---//
//-------------------------//

// Deserialize the Device struct
impl<'de, Protocol> Deserialize<'de> for Device<Protocol>
where
    Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create a new span for the deserialization
        let span = tracing::trace_span!("Device::deserialize");
        let _enter = span.enter();

        // Field identifiers for the Device struct
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            Description,
            Instructions,
            Protocol,
            #[serde(rename = "default_arguments")]
            DefaultArguments,
        }

        // Device visitor
        // Given that Protocol is a generic type, we need to use a PhantomData to specify the type
        struct DeviceVisitor<Protocol>(std::marker::PhantomData<Protocol>);
        impl<'de, Protocol> serde::de::Visitor<'de> for DeviceVisitor<Protocol>
        where
            Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
        {
            // Define the type of value that the visitor will return
            type Value = Device<Protocol>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Create a new span for the expecting message
                let span = tracing::trace_span!("DeviceVisitor::expecting");
                let _enter = span.enter();

                formatter.write_str("struct Device")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                // Create a new span for the visit_map function
                let span = tracing::trace_span!("DeviceVisitor::visit_map");
                let _enter = span.enter();

                // Initialize the fields
                let mut name = String::new();
                let mut description = String::new();
                let mut instructions: BTreeMap<String, Instruction> = BTreeMap::new();
                let mut protocol = None;
                let mut default_arguments = BTreeMap::new();

                // Instruction can be one of 2 types:
                // 1. It can reference a file that contains the instructions (PathBuf)
                //    - The actual deserialization should expect a HashMap<String, PathBuf>
                //    - The HashMap key should be "path"
                //    - The path is then used to read the instructions from the file
                // 2. It can contain the instructions directly (Instruction)
                #[derive(Deserialize, Debug)]
                #[serde(untagged)]
                enum InstructionType {
                    Path(HashMap<String, PathBuf>),
                    Instruction(Instruction),
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

                            // The instructions parameter in the input files is defined as a list
                            let _instructions: Vec<InstructionType> = map.next_value()?;
                            tracing::debug!(instructions_len = %_instructions.len());

                            // Loop through the instructions
                            for (index, instruction) in _instructions.iter().enumerate() {
                                tracing::debug!(index, instruction = ?instruction);
                                match instruction {
                                    InstructionType::Path(file_map) => {
                                        // Get the path to the file
                                        let file = file_map.get("path").ok_or_else(|| {
                                            tracing::error!(
                                                "Field 'path' is missing in the file map"
                                            );
                                            serde::de::Error::missing_field("path")
                                        })?;
                                        // Deserialize the file
                                        let instructions_file: HashMap<String, Vec<Instruction>> =
                                            serde_yaml::from_reader(
                                                std::fs::File::open(file).map_err(|e| {
                                                    tracing::error!("Error reading file: {:?}", e);
                                                    serde::de::Error::custom(e)
                                                })?,
                                            )
                                            .map_err(
                                                |e| {
                                                    tracing::error!(
                                                        "Error deserializing file: {:?}",
                                                        e
                                                    );
                                                    serde::de::Error::custom(e)
                                                },
                                            )?;
                                        // Get the instructions from the file
                                        let instructions_vec = instructions_file
                                            .get("instructions")
                                            .ok_or_else(|| {
                                                tracing::error!(
                                                    "Field 'instructions' is missing in the file"
                                                );
                                                serde::de::Error::missing_field("instructions")
                                            })?
                                            .to_vec();
                                        // Insert the instructions into the instructions HashMap
                                        for instruction in instructions_vec {
                                            if instructions.contains_key(&instruction.name) {
                                                tracing::warn!(
                                                    instruction_name = %instruction.name,
                                                    "Instruction name is duplicated"
                                                );
                                            }
                                            instructions
                                                .insert(instruction.name.clone(), instruction);
                                        }
                                    }
                                    InstructionType::Instruction(instruction) => {
                                        // Insert the instruction into the instructions HashMap
                                        instructions
                                            .insert(instruction.name.clone(), instruction.clone());
                                    }
                                }
                            }

                            // Close the span
                            drop(_enter);
                        }
                        Field::Protocol => {
                            // Create a new span for the Field::Protocol
                            let span = tracing::trace_span!("Field::Protocol");
                            let _enter = span.enter();

                            // Check if the protocol field is duplicated
                            if protocol.is_some() {
                                tracing::error!(
                                    previous_protocol = ?protocol,
                                    new_protocol = ?map.next_value::<Protocol>()?,
                                    "Field 'protocol' is duplicated",
                                );
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }

                            // Get the protocol value
                            protocol = Some(map.next_value()?);
                            tracing::debug!(?protocol);

                            // Close the span
                            drop(_enter);
                        }
                        Field::DefaultArguments => {
                            // Create a new span for the Field::DefaultArguments
                            let span = tracing::trace_span!("Field::DefaultArguments");
                            let _enter = span.enter();

                            // Read the default arguments
                            let _default_arguments: Vec<Arguments> = map.next_value()?;
                            // Insert the default arguments into the default_arguments HashMap
                            for argument in _default_arguments {
                                if default_arguments.contains_key(&argument.name) {
                                    tracing::warn!(
                                        argument_name = %argument.name,
                                        "Argument name is duplicated"
                                    );
                                }
                                tracing::debug!(?argument);
                                default_arguments.insert(argument.name.clone(), argument);
                            }

                            // Close the span
                            drop(_enter);
                        }
                    }
                }

                // Check if mandatory fields are missing
                if name.is_empty() {
                    tracing::error!("Field 'name' is missing");
                    return Err(serde::de::Error::missing_field("name"));
                }
                let protocol = protocol.ok_or_else(|| {
                    tracing::error!("Field 'protocol' is missing");
                    serde::de::Error::missing_field("protocol")
                })?;
                // It is weird to have an empty instructions HashMap
                if instructions.is_empty() {
                    tracing::warn!("Field 'instructions' is empty");
                }

                // Return the Device struct
                Ok(Device {
                    name,
                    description,
                    instructions,
                    protocol,
                    default_arguments,
                })
            }
        }

        // Field identifiers for the Device struct
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "instructions",
            "protocol",
            "default_arguments",
        ];
        // Deserialize the Device struct
        deserializer.deserialize_struct("Device", FIELDS, DeviceVisitor(std::marker::PhantomData))
    }
}

// Deserialize the Arguments struct
impl<'de> Deserialize<'de> for Arguments {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create a new span for the deserialization
        let span = tracing::trace_span!("Arguments::deserialize");
        let _enter = span.enter();

        // Field identifiers for the Arguments struct
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            Value,
        }

        // Arguments visitor
        struct ArgumentsVisitor;

        impl<'de> serde::de::Visitor<'de> for ArgumentsVisitor {
            // The expected type for the Arguments struct
            type Value = Arguments;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Create a new span for the expecting message
                let span = tracing::trace_span!("ArgumentsVisitor::expecting");
                let _enter = span.enter();

                formatter.write_str("struct Arguments")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                // Create a new span for the visit_map function
                let span = tracing::trace_span!("ArgumentsVisitor::visit_map");
                let _enter = span.enter();

                // Initialize the fields
                let mut name = String::new();
                let mut value = None;

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
                            tracing::debug!(name = %name);

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
                                    new_value = ?map.next_value::<Data>()?,
                                    "Field 'value' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("value"));
                            }

                            // Get the value value
                            value = Some(map.next_value()?);

                            // Close the span
                            drop(_enter);
                        }
                    }
                }

                // Check if mandatory fields are missing
                if name.is_empty() {
                    tracing::error!("Field 'name' is missing");
                    return Err(serde::de::Error::missing_field("name"));
                }
                let value = value.ok_or_else(|| {
                    tracing::error!("Field 'value' is missing");
                    serde::de::Error::missing_field("value")
                })?;

                // Return the Arguments struct
                Ok(Arguments { name, value })
            }
        }

        // Field identifiers for the Arguments struct
        const FIELDS: &[&str] = &["name", "value"];
        // Deserialize the Arguments struct
        deserializer.deserialize_struct("Arguments", FIELDS, ArgumentsVisitor)
    }
}
