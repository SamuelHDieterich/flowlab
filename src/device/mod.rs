/*
 ____             _
|  _ \  _____   _(_) ___ ___
| | | |/ _ \ \ / / |/ __/ _ \
| |_| |  __/\ V /| | (_|  __/
|____/ \___| \_/ |_|\___\___|

The device module is used to define the device types that are instruments that can perform instructions through a specific protocol.

*/

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
