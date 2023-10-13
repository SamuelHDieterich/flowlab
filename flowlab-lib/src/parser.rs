/*
 ____
|  _ \ __ _ _ __ ___  ___ _ __
| |_) / _` | '__/ __|/ _ \ '__|
|  __/ (_| | |  \__ \  __/ |
|_|   \__,_|_|  |___/\___|_|

The parser module is used to read/write YAML files and parse them into generic data structures.

*/

// Serde
use serde::Deserialize;

// Filesystem
use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

// Tracing: Logging framework
use tracing::debug;

/// Read a file and return its contents as a string
#[tracing::instrument]
pub async fn read_file(path: impl AsRef<Path> + std::fmt::Debug) -> io::Result<String> {
    // Open the file
    debug!("Reading file");
    let mut file = File::open(path).await?;
    // Buffer to store the file contents
    let mut contents = String::new();
    // Read the file into the buffer
    file.read_to_string(&mut contents).await?;
    debug!("File read");
    // Return the contents
    Ok(contents)
}

/// Error type for the parser
/// This error type is used to wrap all the possible errors that can occur during the parsing process
#[derive(Debug)]
pub enum ParserError {
    /// IO Error: reading the file
    IOError(io::Error),
    /// YAML Error: parsing the YAML file into a generic data structure
    YAMLError(serde_yaml::Error),
}

impl From<io::Error> for ParserError {
    fn from(err: io::Error) -> Self {
        ParserError::IOError(err)
    }
}

impl From<serde_yaml::Error> for ParserError {
    fn from(err: serde_yaml::Error) -> Self {
        ParserError::YAMLError(err)
    }
}

/// Parse a YAML file into a generic data structure
#[tracing::instrument]
pub async fn parse<T>(path: impl AsRef<Path> + std::fmt::Debug) -> Result<T, ParserError>
where
    T: for<'de> Deserialize<'de> + std::fmt::Debug,
{
    debug!("Reading file");
    let contents = read_file(path).await?;
    debug!(
        "Parsing file contents to type {T}",
        T = std::any::type_name::<T>()
    );
    let parsed: T = serde_yaml::from_str(&contents)?;
    Ok(parsed)
}
