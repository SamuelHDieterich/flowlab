// Filesystem
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;

// Tracing: Logging framework
use tracing::debug;

/// The Response struct is used to define the response that a command can return.
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub name: String,
    #[serde(rename = "type", default)]
    pub data_type: DataType,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub values: Vec<Data>,
    #[serde(skip_serializing_if = "str::is_empty", default)]
    pub description: String,
}

impl Response {
    pub fn new(
        name: String,
        data_type: DataType,
        values: Vec<Data>,
        description: String,
    ) -> Self {
        Response {
            name,
            data_type,
            values,
            description,
        }
    }
}

impl Default for Response {
    fn default() -> Self {
        Response {
            name: String::new(),
            data_type: DataType::String,
            values: Vec::new(),
            description: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Data {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Boolean,
    Integer,
    Float,
    String,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::String
    }
}

/// Error type for the parser.
/// This error type is used to wrap all the possible errors that can occur during the parsing process.
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

/// Read a file and return its contents as a string
///
/// # Arguments
///
/// - `path`
///   A path to the file to read
///
/// # Returns
///
/// The contents of the file as a string
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

// Parse a YAML file into a generic data structure
//
// # Arguments
//
// - `path`
//   A path to the file to parse
//
// # Returns
//
// The parsed data structure of type `T`
#[tracing::instrument]
async fn parse<T>(path: impl AsRef<Path> + std::fmt::Debug) -> Result<T, ParserError>
where
    T:for<'de> serde::de::Deserialize<'de> + std::fmt::Debug,
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

#[tokio::main]
async fn main() {
    // // Create a new response
    // let response = Response::new("name", DataType::String, vec![], "");
    // println!("{:?}", response);
    // // Serialize the response to YAML
    // let response_yaml = serde_yaml::to_string(&response).unwrap();
    // println!("{}", response_yaml);
    // // Deserialize the response from YAML
    // let response2: Response = serde_yaml::from_str(&response_yaml).unwrap();
    // println!("{:?}", response2);

    // Parse a YAML file into a generic data structure
    let path = PathBuf::from("./config/instructions/response.yaml");
    let content = read_file(&path).await.unwrap();
    let response_path: Response = serde_yaml::from_str(&content).unwrap();
    println!("{:?}", response_path);

    // Parse a YAML file into a generic data structure
    let response_from_parse: Response = parse(&path).await.unwrap();
    println!("{:?}", response_from_parse);

    // Parse a YAML file into a generic data structure
    let reader = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
    let response_from_file: Response = serde_yaml::from_reader(reader).unwrap();
    println!("{:?}", response_from_file);
}
