// Base device module
mod base;
pub use crate::device::base::{Device, Query};

// TCP device module
#[cfg(feature = "tcp")]
mod tcp;
#[cfg(feature = "tcp")]
pub use crate::device::tcp::TCP;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Protocols {
    #[cfg(feature = "tcp")]
    TCP(TCP),
}
