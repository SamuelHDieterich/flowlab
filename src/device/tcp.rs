//! # TCP
//! TCP protocol implementation for `Device`.
//!
//! This module contains the TCP protocol implementation for the `Device` struct.
//! TCP is a well-known communication protocol that can be used to communicate with devices over a network connection (e.g., Ethernet).
//! The `Protocol` requires an IP address and a port to communicate with the device.
//!
//! The `tcp` feature must be enabled to use this module.

//----------------//
//---  CRATES  ---//
//----------------//

// Internal modules
//// The `Query` trait is used to send commands and receive responses from the device.
use super::Query;

// External crates
//// Allows traits to have async functions
use async_trait::async_trait;
//// Serde: Serialization/Deserialization framework
use serde::Deserialize;
//// Async TCP implementation
use std::net::IpAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

//-----------------//
//---  STRUCTS  ---//
//-----------------//

/// TCP specific fields
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TCP {
    /// IP address of the device
    pub ip: IpAddr,
    /// Port of the device
    pub port: u16,
}

//------------------------//
//---  IMPLEMENATIONS  ---//
//------------------------//

impl TCP {
    /// Create a new TCP struct
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self { ip, port }
    }
}

impl Default for TCP {
    fn default() -> Self {
        Self {
            ip: IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
            port: 0,
        }
    }
}

#[async_trait]
impl Query for TCP {
    #[tracing::instrument]
    async fn query(&self, command: &str) -> Result<Option<String>, std::io::Error> {
        tracing::trace!("Connecting to device");
        let mut stream = TcpStream::connect((self.ip, self.port)).await?;
        tracing::trace!("Sending command");
        stream.write_all(command.as_bytes()).await?;
        let mut buffer = [0; 1024];
        tracing::trace!("Saving response to buffer");
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            tracing::info!("No response from device");
            return Ok(None);
        }
        tracing::trace!("Converting buffer to string");
        let response = String::from_utf8_lossy(&buffer[..n]).to_string();
        Ok(Some(response))
    }
}
