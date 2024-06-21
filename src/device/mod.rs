//! # Device
//! Device module is used to define the device types that are instruments that can perform instructions through a specific protocol.
//!
//! Specific protocols can be enabled through features. For instance, the `TCP` protocol can be enabled with the `tcp` feature.
//! The `Protocols` enum is used to define the protocols that a device can use, according to the enabled features.

//-----------------//
//---  MODULES  ---//
//-----------------//

// Base device module
mod base;
pub use crate::device::base::*;

// Serde implementations
mod deserialize;

// TCP device module
#[cfg(feature = "tcp")]
mod tcp;
#[cfg(feature = "tcp")]
pub use crate::device::tcp::TCP;

//----------------//
//---  CRATES  ---//
//----------------//

// Allows traits to have async functions
// This is required for the Query trait for Rust version 1.75 or below and, for now, it is recommended for public traits.
use async_trait::async_trait;

// Serde: Serialization/Deserialization framework
use serde::Deserialize;

//----------------//
//---  TRAITS  ---//
//----------------//

/// A protocol must implement the Query trait which allows the device to send commands and receive responses.
#[async_trait]
pub trait Query {
    /// Send a command to the device and return the response (if any).
    async fn query(&self, command: &str) -> Result<Option<String>, std::io::Error>;
}

//---------------//
//---  ENUMS  ---//
//---------------//

/// The Protocols enum is used to define the protocols that a device can use.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Protocols {
    #[cfg(feature = "tcp")]
    TCP(TCP),
}

//-------------------------//
//---  IMPLEMENTATIONS  ---//
//-------------------------//

impl Default for Protocols {
    fn default() -> Self {
        #[cfg(feature = "tcp")]
        return Protocols::TCP(TCP::default());
    }
}

#[async_trait]
impl Query for Protocols {
    #[tracing::instrument]
    async fn query(&self, command: &str) -> Result<Option<String>, std::io::Error> {
        match self {
            #[cfg(feature = "tcp")]
            Protocols::TCP(tcp) => tcp.query(command).await,
        }
    }
}
