pub mod device;
pub use crate::device::Device;
#[cfg(feature = "tcp")]
pub use crate::device::TCP;
