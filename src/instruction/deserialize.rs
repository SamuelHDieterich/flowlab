//----------------//
//---  CRATES  ---//
//----------------//

// Internal modules
use super::base::*;
use crate::{Data, DataType};

// Built-in modules
//// HashMap: Data structure for storing key-value pairs
use std::collections::HashMap;

// External crates
//// Serde: Serialization/Deserialization framework
use serde::{ser::SerializeStruct, Deserialize};

//-------------------------//
//---  IMPLEMENTATIONS  ---//
//-------------------------//

// Deserialize the Instruction struct
impl<'de> Deserialize<'de> for Instruction {
    fn deserialize<D>(deserializer: D) -> Result<Instruction, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create a new span for the deserialization
        let span = tracing::trace_span!("Instruction::deserialize");
        let _enter = span.enter();

        // Field identifiers for the Instruction struct
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            Description,
            Command,
            Response,
        }

        // Instruction visitor
        struct InstructionVisitor;
        impl<'de> serde::de::Visitor<'de> for InstructionVisitor {
            // Define the type of value that the visitor will return
            type Value = Instruction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Create a new span for the expecting message
                let span = tracing::trace_span!("InstructionVisitor::expecting");
                let _enter = span.enter();

                formatter.write_str("struct Instruction")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Instruction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                // Create a new span for the visit_map function
                let span = tracing::trace_span!("InstructionVisitor::visit_map");
                let _enter = span.enter();

                // Initialize the fields of the Instruction struct
                let mut name = String::new();
                let mut description = String::new();
                let mut command = None;
                let mut response = None;

                // Loop through the fields of the Instruction struct
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
                        Field::Command => {
                            // Create a new span for the Field::Command
                            let span = tracing::trace_span!("Field::Command");
                            let _enter = span.enter();

                            // Check if the command field is duplicated
                            if command.is_some() {
                                tracing::error!(
                                    previous_command = ?command,
                                    new_command = ?map.next_value::<Command>()?,
                                    "Field 'command' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("command"));
                            }

                            // Get the command value
                            command = Some(map.next_value()?);
                            tracing::debug!(?command);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Response => {
                            // Create a new span for the Field::Response
                            let span = tracing::trace_span!("Field::Response");
                            let _enter = span.enter();

                            // Check if the response field is duplicated
                            if response.is_some() {
                                tracing::error!(
                                    previous_response = ?response,
                                    new_response = ?map.next_value::<Response>()?,
                                    "Field 'response' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("response"));
                            }

                            // Get the response value
                            response = Some(map.next_value()?);
                            tracing::debug!(?response);

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
                let command = command.ok_or_else(|| {
                    tracing::error!("Field 'command' is missing");
                    serde::de::Error::missing_field("command")
                })?;

                // Create a new Instruction struct
                Ok(Instruction {
                    name,
                    description,
                    command,
                    response,
                })
            }
        }

        // Field identifiers for the Instruction struct
        const FIELDS: &[&str] = &["name", "description", "command", "response"];
        // Deserialize the Instruction struct
        deserializer.deserialize_struct("Instruction", FIELDS, InstructionVisitor)
    }
}

// Serialize the Instruction struct
impl serde::ser::Serialize for Instruction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Create a new span for the serialization
        let span = tracing::trace_span!("Instruction::serialize");
        let _enter = span.enter();

        // Create a new struct for the Instruction struct
        let mut state = serializer.serialize_struct("Instruction", 4)?;

        // Serialize the name field
        state.serialize_field("name", &self.name)?;

        // Serialize the description field
        state.serialize_field("description", &self.description)?;

        // Serialize the command field
        state.serialize_field("command", &self.command)?;

        // Serialize the response field
        state.serialize_field("response", &self.response)?;

        // Close the struct
        state.end()
    }
}

// Deserialize the Command struct
impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Command, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create a new span for the deserialization
        let span = tracing::trace_span!("Command::deserialize");
        let _enter = span.enter();

        // Field identifiers for the Command struct
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Query,
            Parameters,
        }

        // Command visitor
        struct CommandVisitor;
        impl<'de> serde::de::Visitor<'de> for CommandVisitor {
            // Define the type of value that the visitor will return
            type Value = Command;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Create a new span for the expecting message
                let span = tracing::trace_span!("CommandVisitor::expecting");
                let _enter = span.enter();

                formatter.write_str("struct Command")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Command, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                // Create a new span for the visit_map function
                let span = tracing::trace_span!("CommandVisitor::visit_map");
                let _enter = span.enter();

                // Initialize the fields of the Command struct
                let mut query = String::new();
                let mut parameters = HashMap::new();

                // Loop through the fields of the Command struct
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Query => {
                            // Create a new span for the Field::Query
                            let span = tracing::trace_span!("Field::Query");
                            let _enter = span.enter();

                            // Check if the query field is duplicated
                            if !query.is_empty() {
                                tracing::error!(
                                    previous_query = %query,
                                    new_query = %map.next_value::<String>()?,
                                    "Field 'query' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("query"));
                            }

                            // Get the query value
                            query = map.next_value()?;
                            tracing::debug!(%query);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Parameters => {
                            // Create a new span for the Field::Parameters
                            let span = tracing::trace_span!("Field::Parameters");
                            let _enter = span.enter();

                            // The parameters parameter in the input files is defined as a list
                            let _parameters: Vec<Parameter> = map.next_value()?;
                            tracing::debug!(parameters_len = %_parameters.len());

                            // Convert the list of parameters to a HashMap
                            for p in _parameters {
                                if parameters.contains_key(&p.name) {
                                    tracing::warn!(
                                        parameter_name = %p.name,
                                        "Parameter name is duplicated"
                                    );
                                }
                                parameters.insert(p.name.clone(), p);
                            }

                            // Close the span
                            drop(_enter);
                        }
                    }
                }

                // Check if mandatory fields are missing
                if query.is_empty() {
                    tracing::error!("Field 'query' is missing");
                    return Err(serde::de::Error::missing_field("query"));
                }

                // Create a new Command struct
                Ok(Command { query, parameters })
            }
        }

        // Field identifiers for the Command struct
        const FIELDS: &[&str] = &["query", "parameters"];
        // Deserialize the Command struct
        deserializer.deserialize_struct("Command", FIELDS, CommandVisitor)
    }
}

// Serialize the Command struct
impl serde::ser::Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Create a new span for the serialization
        let span = tracing::trace_span!("Command::serialize");
        let _enter = span.enter();

        // Create a new struct for the Command struct
        let mut state = serializer.serialize_struct("Command", 2)?;

        // Serialize the query field
        state.serialize_field("query", &self.query)?;

        // Serialize the parameters field
        // The parameters field is a HashMap, so it needs to be serialized as a list
        let parameters: Vec<&Parameter> = self.parameters.values().collect();
        state.serialize_field("parameters", &parameters)?;

        // Close the struct
        state.end()
    }
}

// Deserialize the Parameter struct
impl<'de> Deserialize<'de> for Parameter {
    fn deserialize<D>(deserializer: D) -> Result<Parameter, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create a new span for the deserialization
        let span = tracing::trace_span!("Parameter::deserialize");
        let _enter = span.enter();

        // Field identifiers for the Parameter struct
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            Type,
            Values,
            Default,
            Description,
        }

        // Parameter visitor
        struct ParameterVisitor;
        impl<'de> serde::de::Visitor<'de> for ParameterVisitor {
            // Define the type of value that the visitor will return
            type Value = Parameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Create a new span for the expecting message
                let span = tracing::trace_span!("ParameterVisitor::expecting");
                let _enter = span.enter();

                formatter.write_str("struct Parameter")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Parameter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                // Create a new span for the visit_map function
                let span = tracing::trace_span!("ParameterVisitor::visit_map");
                let _enter = span.enter();

                // Initialize the fields of the Parameter struct
                let mut name = String::new();
                let mut data_type = DataType::String;
                let mut values = Vec::new();
                let mut default = None;
                let mut description = String::new();

                // Loop through the fields of the Parameter struct
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
                        Field::Type => {
                            // Create a new span for the Field::Type
                            let span = tracing::trace_span!("Field::Type");
                            let _enter = span.enter();

                            // Check if the type field is duplicated
                            if data_type != DataType::String {
                                tracing::error!(
                                    previous_type = ?data_type,
                                    new_type = ?map.next_value::<DataType>()?,
                                    "Field 'type' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("type"));
                            }

                            // Get the type value
                            data_type = map.next_value()?;
                            tracing::debug!(?data_type);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Values => {
                            // Create a new span for the Field::Values
                            let span = tracing::trace_span!("Field::Values");
                            let _enter = span.enter();

                            // Get the values value
                            values = map.next_value()?;
                            tracing::debug!(values_len = %values.len());

                            // Close the span
                            drop(_enter);
                        }
                        Field::Default => {
                            // Create a new span for the Field::Default
                            let span = tracing::trace_span!("Field::Default");
                            let _enter = span.enter();

                            // Check if the default field is duplicated
                            if default.is_some() {
                                tracing::error!(
                                    previous_default = ?default,
                                    new_default = ?map.next_value::<Data>()?,
                                    "Field 'default' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("default"));
                            }

                            // Get the default value
                            default = Some(map.next_value()?);
                            tracing::debug!(?default);

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
                    }
                }

                // Check if mandatory fields are missing
                if name.is_empty() {
                    tracing::error!("Field 'name' is missing");
                    return Err(serde::de::Error::missing_field("name"));
                }

                // Create a new Parameter struct
                Ok(Parameter {
                    name,
                    data_type,
                    values,
                    default,
                    description,
                })
            }
        }

        // Field identifiers for the Parameter struct
        const FIELDS: &[&str] = &["name", "type", "values", "default", "description"];
        // Deserialize the Parameter struct
        deserializer.deserialize_struct("Parameter", FIELDS, ParameterVisitor)
    }
}

// Serialize the Parameter struct
impl serde::ser::Serialize for Parameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Create a new span for the serialization
        let span = tracing::trace_span!("Parameter::serialize");
        let _enter = span.enter();

        // Create a new struct for the Parameter struct
        let mut state = serializer.serialize_struct("Parameter", 5)?;

        // Serialize the name field
        state.serialize_field("name", &self.name)?;

        // Serialize the data_type field
        state.serialize_field("type", &self.data_type)?;

        // Serialize the values field
        state.serialize_field("values", &self.values)?;

        // Serialize the default field
        state.serialize_field("default", &self.default)?;

        // Serialize the description field
        state.serialize_field("description", &self.description)?;

        // Close the struct
        state.end()
    }
}

// Deserialize the Response struct
impl<'de> Deserialize<'de> for Response {
    fn deserialize<D>(deserializer: D) -> Result<Response, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create a new span for the deserialization
        let span = tracing::trace_span!("Response::deserialize");
        let _enter = span.enter();

        // Field identifiers for the Response struct
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Format,
            Parameters,
        }

        // Response visitor
        struct ResponseVisitor;
        impl<'de> serde::de::Visitor<'de> for ResponseVisitor {
            // Define the type of value that the visitor will return
            type Value = Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Create a new span for the expecting message
                let span = tracing::trace_span!("ResponseVisitor::expecting");
                let _enter = span.enter();

                formatter.write_str("struct Response")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Response, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                // Create a new span for the visit_map function
                let span = tracing::trace_span!("ResponseVisitor::visit_map");
                let _enter = span.enter();

                // Initialize the fields of the Response struct
                let mut format = String::new();
                let mut parameters = HashMap::new();

                // Loop through the fields of the Response struct
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Format => {
                            // Create a new span for the Field::Format
                            let span = tracing::trace_span!("Field::Format");
                            let _enter = span.enter();

                            // Check if the format field is duplicated
                            if !format.is_empty() {
                                tracing::error!(
                                    previous_format = %format,
                                    new_format = %map.next_value::<String>()?,
                                    "Field 'format' is duplicated"
                                );
                                return Err(serde::de::Error::duplicate_field("format"));
                            }

                            // Get the format value
                            format = map.next_value()?;
                            tracing::debug!(%format);

                            // Close the span
                            drop(_enter);
                        }
                        Field::Parameters => {
                            // Create a new span for the Field::Parameters
                            let span = tracing::trace_span!("Field::Parameters");
                            let _enter = span.enter();

                            // The parameters parameter in the input files is defined as a list
                            let _parameters: Vec<Parameter> = map.next_value()?;
                            tracing::debug!(parameters_len = %_parameters.len());

                            // Convert the list of parameters to a HashMap
                            for p in _parameters {
                                if parameters.contains_key(&p.name) {
                                    tracing::warn!(
                                        parameter_name = %p.name,
                                        "Parameter name is duplicated"
                                    );
                                }
                                parameters.insert(p.name.clone(), p);
                            }

                            // Close the span
                            drop(_enter);
                        }
                    }
                }

                // Check if mandatory fields are missing
                if format.is_empty() {
                    tracing::error!("Field 'format' is missing");
                    return Err(serde::de::Error::missing_field("format"));
                }

                let _pattern = Response::create_pattern(&format, &parameters).map_err(|e| {
                    tracing::error!(error = %e, "Failed to create pattern");
                    serde::de::Error::custom(e)
                })?;

                // Create a new Response struct
                Ok(Response {
                    format,
                    parameters,
                    _pattern,
                })
            }
        }

        // Field identifiers for the Response struct
        const FIELDS: &[&str] = &["format", "parameters"];
        // Deserialize the Response struct
        deserializer.deserialize_struct("Response", FIELDS, ResponseVisitor)
    }
}

// Serialize the Response struct
impl serde::ser::Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Create a new span for the serialization
        let span = tracing::trace_span!("Response::serialize");
        let _enter = span.enter();

        // Create a new struct for the Response struct
        let mut state = serializer.serialize_struct("Response", 2)?;

        // Serialize the format field
        state.serialize_field("format", &self.format)?;

        // Serialize the parameters field
        // The parameters field is a HashMap, so it needs to be serialized as a list
        let parameters: Vec<&Parameter> = self.parameters.values().collect();
        state.serialize_field("parameters", &parameters)?;

        // Close the struct
        state.end()
    }
}
