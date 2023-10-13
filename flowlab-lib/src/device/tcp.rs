/*
 _____ ____ ____
|_   _/ ___|  _ \
  | || |   | |_) |
  | || |___|  __/
  |_| \____|_|

TCP Protocol implementation.

*/

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

/// TCP specific fields
#[derive(Debug, Serialize, Deserialize)]
pub struct TCP {
    /// IP address of the device
    pub ip: IpAddr,
    /// Port of the device
    pub port: u16,
}

impl Device<TCP> {
    /// Create a new TCP device
    pub fn new(name: &str, ip: IpAddr, port: u16) -> Self {
        Device {
            name: name.to_string(),
            instruction: vec![],
            protocol: TCP { ip, port },
        }
    }
}

#[async_trait]
impl Query for Device<TCP> {
    async fn query(&self, command: &str) -> Result<Option<String>, std::io::Error> {
        let mut stream = TcpStream::connect((self.protocol.ip, self.protocol.port)).await?;
        stream.write_all(command.as_bytes()).await?;
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            return Ok(None);
        }
        let response = String::from_utf8_lossy(&buffer[..n]).to_string();
        Ok(Some(response))
    }
}
