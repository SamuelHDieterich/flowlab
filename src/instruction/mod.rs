//! # Instruction
//! The instruction module is used to define the instructions that a device can perform.

// Base instruction module
pub mod base;
pub use crate::instruction::base::*;

// Pipeline instruction module
pub mod pipeline;
pub use crate::instruction::pipeline::*;

// Format instruction module
pub mod format;
pub use crate::instruction::format::*;
