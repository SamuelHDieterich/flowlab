// Base device module
mod base;
pub use crate::device::base::Device;

// TCP device module
#[cfg(feature = "tcp")]
mod tcp;
#[cfg(feature = "tcp")]
pub use crate::device::tcp::TCP;
