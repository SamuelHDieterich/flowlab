//! # Pipeline
//! The pipeline module contains the code responsible to parse the pipeline instructions.

//-----------------//
//---  MODULES  ---//
//-----------------//

// Base pipeline module
pub mod base;
pub use crate::pipeline::base::*;

// Serde implementations
mod deserialize;

// Run pipeline module
pub mod run;
