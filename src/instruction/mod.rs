//! # Instruction
//! The instruction module is used to define the instructions that a device can perform.

// Base instruction module
pub mod base;
pub use crate::instruction::base::*;

// Format instruction module
pub mod format;
pub use crate::instruction::format::*;

/// Get an instruction from a vector of instructions by its name
/// Also consider the aliases of the instruction
fn find_instruction_with_name<'a>(
    instructions: &'a Vec<Instruction>,
    name: &str,
) -> Option<&'a Instruction> {
    instructions
        .iter()
        .find(|instruction| instruction.name == name)
}
