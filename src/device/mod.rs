//! # Device
//! Device module is used to define the device types that are instruments that can perform instructions through a specific protocol.
//!
//! Specific protocols can be enabled through features. For instance, the `TCP` protocol can be enabled with the `tcp` feature.
//! The `Protocols` enum is used to define the protocols that a device can use, according to the enabled features.

// Base device module
mod base;
pub use crate::device::base::*;

// TCP device module
#[cfg(feature = "tcp")]
mod tcp;
#[cfg(feature = "tcp")]
pub use crate::device::tcp::TCP;

use serde::{Deserialize, Serialize};

/// The Protocols enum is used to define the protocols that a device can use.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Protocols {
    #[cfg(feature = "tcp")]
    TCP(TCP),
}
