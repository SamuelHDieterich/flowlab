//! # Format
//! Responsible for formatting the commands in the instructions with the appropriate parameters.
//!
//! The `format` module is used to format the commands in the instructions with the appropriate parameters.
//! It contains functions to convert a hashmap/dictionary to a context and to format a command with the given parameters.

//----------------//
//---  CRATES  ---//
//----------------//

// Base instruction implementation
use super::base::Command;

// Serde: Serialization/Deserialization framework
use serde::Serialize;

//-------------------//
//---  FUNCTIONS  ---//
//-------------------//

#[tracing::instrument]
pub fn format_command<T>(command: &Command, parameters: &T) -> Result<String, tera::Error>
where
    T: Serialize + std::fmt::Debug,
{
    // Convert the parameters to a context
    let context = tera::Context::from_serialize(parameters)?;
    let mut tera = tera::Tera::default();
    tera.add_raw_template("command", &command.query)?;
    tracing::debug!("Formatting command");
    tera.render("command", &context)
}
