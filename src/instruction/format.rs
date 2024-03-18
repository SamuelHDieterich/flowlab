/*
 _____                          _
|  ___|__  _ __ _ __ ___   __ _| |_
| |_ / _ \| '__| '_ ` _ \ / _` | __|
|  _| (_) | |  | | | | | | (_| | |_
|_|  \___/|_|  |_| |_| |_|\__,_|\__|

The format module is responsible for formatting the commands in the instructions with the appropriate parameters.

*/

// HashMap: Data structure for storing key-value pairs
use std::collections::HashMap;

// Base instruction implementation
use super::base::Command;

// Tera: Template engine
use tera::{Context, Tera};

// Tracing: Logging framework
use tracing::debug;

/// Convert a hashmap to a context
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
#[tracing::instrument]
pub fn format_command(
    command: &Command,
    parameters: &HashMap<String, String>,
) -> Result<String, tera::Error> {
    debug!("Converting hashmap to context");
    let context = hashmap_to_context(&parameters);
    let mut tera = Tera::default();
    tera.add_raw_template("command", &command.query)?;
    debug!("Formatting command");
    tera.render("command", &context)
}
