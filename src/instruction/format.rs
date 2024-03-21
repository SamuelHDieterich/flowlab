//! # Format
//! Responsible for formatting the commands in the instructions with the appropriate parameters.
//!
//! The `format` module is used to format the commands in the instructions with the appropriate parameters.
//! It contains functions to convert a hashmap/dictionary to a context and to format a command with the given parameters.

// HashMap: Data structure for storing key-value pairs
use std::collections::HashMap;

// Base instruction implementation
use super::base::Command;

// Tera: Template engine
use tera::{Context, Tera};

// Tracing: Logging framework
use tracing::{debug, warn};

/// Convert a hashmap to a context
///
/// # Arguments
///
/// - `hashmap`
///   A `HashMap` with the parameters, the key is the parameter name and the value is the parameter value.
///
/// # Returns
///
/// A context, Tera struct, with the parameters to be used in the template rendering.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use flowlab::instruction::format::hashmap_to_context;
///
/// let mut hashmap = HashMap::new();
/// hashmap.insert("param1".to_string(), "value1".to_string());
/// hashmap.insert("param2".to_string(), "value2".to_string());
/// let context = hashmap_to_context(&hashmap);
/// ```
#[tracing::instrument]
pub fn hashmap_to_context(hashmap: &HashMap<String, String>) -> Context {
    debug!("Converting hashmap to context");
    let mut context = Context::new();
    for (key, value) in hashmap {
        context.insert(key, value);
    }
    context
}

/// Format a command with the given parameters
///
/// # Arguments
///
/// - `command`
///   A `Command` struct to be formatted.
/// - `parameters`
///   A `HashMap` with the parameters, the key is the parameter name and the value is the parameter value.
///
/// # Returns
///
/// A `String` with the formatted `command`.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use flowlab::instruction::{base::{Parameters, Command}, format::format_command};
///
/// let command = Command {
///    query: "echo {{ param1 }} {{ param2 }}".to_string(),
///    parameters: Some(vec![
///        Parameters {
///            name: "param1".to_string(),
///            data_type: "string".to_string(),
///            values: None,
///            default: None,
///            description: None,
///        },
///        Parameters {
///            name: "param2".to_string(),
///            data_type: "string".to_string(),
///            values: None,
///            default: None,
///            description: None,
///        },
///    ])
/// };
/// let mut parameter = HashMap::new();
/// parameter.insert("param1".to_string(), "value1".to_string());
/// parameter.insert("param2".to_string(), "value2".to_string());
/// let context = format_command(&command, &parameter);
/// ```
#[tracing::instrument]
pub fn format_command(
    command: &Command,
    parameters: &HashMap<String, String>,
) -> Result<String, tera::Error> {
    debug!("Converting hashmap to context");
    // Check if the command has Parameters
    if command.parameters.is_none() {
        warn!("Command has no parameters");
        return Ok(command.query.clone());
    }
    // Check if the parameters are empty
    if parameters.is_empty() {
        warn!("Parameters are empty");
        return Ok(command.query.clone());
    }
    // Check if length of parameters is equal to the length of the command parameters
    if command.parameters.as_ref().unwrap().len() != parameters.len() {
        warn!("Length of parameters is not equal to the length of the command parameters");
        return Err(tera::Error::msg(
            "Length of parameters is not equal to the length of the command parameters",
        ));
    }
    // Convert the parameters to a context
    let context = hashmap_to_context(&parameters);
    let mut tera = Tera::default();
    tera.add_raw_template("command", &command.query)?;
    debug!("Formatting command");
    tera.render("command", &context)
}
