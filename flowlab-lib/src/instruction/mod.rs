/*
 ___           _                   _   _
|_ _|_ __  ___| |_ _ __ _   _  ___| |_(_) ___  _ __
 | || '_ \/ __| __| '__| | | |/ __| __| |/ _ \| '_ \
 | || | | \__ \ |_| |  | |_| | (__| |_| | (_) | | | |
|___|_| |_|___/\__|_|   \__,_|\___|\__|_|\___/|_| |_|

The instruction module is used to define the instructions that a device can perform.

*/

// Base instruction module
pub mod base;
pub use crate::instruction::base::*;

// Format instruction module
pub mod format;
pub use crate::instruction::format::*;
