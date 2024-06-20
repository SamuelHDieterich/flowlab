//! # Instruction
//! The instruction module is used to define the instructions that a device can perform.

//-----------------//
//---  MODULES  ---//
//-----------------//

// Base instruction module
pub mod base;
pub use crate::instruction::base::*;

// Format instruction module
pub mod format;
pub use crate::instruction::format::*;
