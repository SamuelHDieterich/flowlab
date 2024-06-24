//! # Format
//! Responsible for formatting the commands in the instructions with the appropriate parameters.
//!
//! The `format` module is used to format the commands in the instructions with the appropriate parameters.
//! It contains functions to convert a hashmap/dictionary to a context and to format a command with the given parameters.

//----------------//
//---  CRATES  ---//
//----------------//

// Internal modules
//// Base instruction implementation
use super::{Command, Parameter, Response};
use crate::{Data, DataType};

// Built-in modules
//// Basic data structures
use std::collections::HashMap;

// External crates
//// Serde: Serialization/Deserialization framework
use serde::Serialize;
//// Regex: Regular expressions
use regex::Regex;

//-------------------------//
//---  IMPLEMENTATIONS  ---//
//-------------------------//

impl Command {
    /// Render the command with the given parameters.
    #[tracing::instrument(level = "debug")]
    pub fn render<T>(&self, parameters: &T) -> Result<String, tera::Error>
    where
        T: Serialize + std::fmt::Debug,
    {
        // Convert the parameters to a context
        let context = tera::Context::from_serialize(parameters)?;
        let mut tera = tera::Tera::default();
        tera.add_raw_template("command", &self.query)?;
        tracing::debug!("Formatting command");
        tera.render("command", &context)
    }
}

impl Response {
    /// Create a pattern from the format and parameters.
    /// This function replaces the parameters in the format with the appropriate regex pattern. That way, the response can be parsed and its values extracted.
    #[tracing::instrument(level = "debug")]
    pub(crate) fn create_pattern(
        format: &str,
        parameters: &HashMap<String, Parameter>,
    ) -> Result<Regex, regex::Error> {
        // Create a copy of the format
        let mut pattern = format.to_string();
        // Iterate over the parameters
        for (name, parameter) in parameters {
            // The parameter format must be {{parameter_name}}
            let to = format!("{{{{{}}}}}", name);
            // The regex pattern must be (?<parameter_name>regex_pattern)
            // The regex pattern depends on the data type of the parameter
            let from = format!(
                "(?<{}>{})",
                name,
                match parameter.data_type {
                    DataType::Boolean => "true|false|0|1", // Boolean values can be true, false, 0, or 1
                    DataType::Integer => "\\d+",
                    DataType::Float => "\\d+\\.\\d+", // Float values can be integers or floats, but the decimal point is required
                    DataType::String => "\\S+",
                }
            );
            tracing::debug!("Replacing {} with {}", to, from);
            pattern = pattern.replace(&to, &from);
        }
        Ok(Regex::new(&pattern)?)
    }

    /// Parse the response and return a hashmap with the extracted values.
    #[tracing::instrument(level = "debug")]
    pub fn parse(&self, response: &str) -> Result<HashMap<String, Data>, String> {
        let mut parsed_data = HashMap::new();
        if let Some(captures) = self._pattern.captures(response) {
            for name in self._pattern.capture_names().flatten() {
                if let Some(value) = captures.name(name) {
                    let value = value.as_str();
                    let parameter = self.parameters.get(name).ok_or_else(|| {
                        tracing::error!("Parameter {} not found", name);
                        format!("Parameter {} not found", name)
                    })?;
                    let data = match parameter.data_type {
                        DataType::Boolean => Data::Boolean(value.parse().map_err(|_| {
                            tracing::error!("Invalid boolean value: {}", value);
                            format!("Invalid boolean value: {}", value)
                        })?),
                        DataType::Integer => Data::Integer(value.parse().map_err(|_| {
                            tracing::error!("Invalid integer value: {}", value);
                            format!("Invalid integer value: {}", value)
                        })?),
                        DataType::Float => Data::Float(value.parse().map_err(|_| {
                            tracing::error!("Invalid float value: {}", value);
                            format!("Invalid float value: {}", value)
                        })?),
                        DataType::String => Data::String(value.to_string()),
                    };
                    parsed_data.insert(name.to_string(), data);
                }
            }
        }
        Ok(parsed_data)
    }
}
