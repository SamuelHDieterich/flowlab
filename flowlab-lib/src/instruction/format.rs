/*
 _____                          _
|  ___|__  _ __ _ __ ___   __ _| |_
| |_ / _ \| '__| '_ ` _ \ / _` | __|
|  _| (_) | |  | | | | | | (_| | |_
|_|  \___/|_|  |_| |_| |_|\__,_|\__|

The format module is responsible for formatting the commands in the instructions with the appropriate parameters.

*/

use std::collections::HashMap;

use super::base::Command;
use tera::{Context, Tera};

/// Convert a hashmap to a context
pub fn hashmap_to_context(hashmap: &HashMap<String, String>) -> Context {
    let mut context = Context::new();
    for (key, value) in hashmap {
        context.insert(key, value);
    }
    context
}

/// Format a command with the given parameters
pub fn format_command(
    command: &Command,
    parameters: &HashMap<String, String>,
) -> Result<String, tera::Error> {
    let context = hashmap_to_context(&parameters);
    let mut tera = Tera::default();
    tera.add_raw_template("command", &command.query)?;
    tera.render("command", &context)
}
