//! # TCP
//! TCP protocol implementation for `Device`.
//!
//! This module contains the TCP protocol implementation for the `Device` struct.
//! TCP is a well-known communication protocol that can be used to communicate with devices over a network connection (e.g., Ethernet).
//! The `Protocol` requires an IP address and a port to communicate with the device.
//!
//! The `tcp` feature must be enabled to use this module.

// Base device implementation
use super::base::{Device, Query};

// Allows traits to have async functions
use async_trait::async_trait;

// Serde: Serialization/Deserialization framework
use serde::{Deserialize, Serialize};

// Async TCP implementation
use std::net::IpAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

// Tracing: Logging framework
use tracing::{debug, info};

/// TCP specific fields
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TCP {
    /// IP address of the device
    pub ip: IpAddr,
    /// Port of the device
    pub port: u16,
}

#[async_trait]
impl Query for Device<TCP> {
    #[tracing::instrument]
    async fn query(&self, command: &str) -> Result<Option<String>, std::io::Error> {
        debug!("Connecting to device");
        let mut stream = TcpStream::connect((self.protocol.ip, self.protocol.port)).await?;
        debug!("Sending command");
        stream.write_all(command.as_bytes()).await?;
        let mut buffer = [0; 1024];
        debug!("Saving response to buffer");
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            info!("No response from device");
            return Ok(None);
        }
        debug!("Converting buffer to string");
        let response = String::from_utf8_lossy(&buffer[..n]).to_string();
        Ok(Some(response))
    }
}
