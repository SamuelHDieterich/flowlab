//! # Parser
//! The parser module is used to read/write YAML files and parse them into generic data structures.

// Serde
use serde::Deserialize;

// Filesystem
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

// Tracing: Logging framework
use tracing::debug;

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

/// Parse a YAML file into a generic data structure
///
/// # Arguments
///
/// - `path`
///   A path to the file to parse
///
/// # Returns
///
/// The parsed data structure of type `T`
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

/// Given a list of files and folders, parse them into a list of type `T`
///
/// # Arguments
///
/// - `files`
///  A list of files and folders to parse
///
/// # Returns
///
/// A list of parsed data structures of type `T`
#[tracing::instrument]
pub async fn parse_files<T>(files: Vec<PathBuf>) -> Vec<T>
where
    T: for<'de> serde::de::Deserialize<'de> + std::fmt::Debug,
{
    Box::pin(async move {
        let mut parsed_files: Vec<T> = Vec::new();
        for file in files {
            debug!("Parsing file: {:?}", file);
            // If file is a folder, call this function recursively
            if file.is_dir() {
                debug!("Folder detected, parsing files in folder");
                let mut folder_files: Vec<PathBuf> = Vec::new();
                for entry in std::fs::read_dir(file).unwrap() {
                    let entry = entry.unwrap();
                    folder_files.push(entry.path());
                }
                parsed_files.append(&mut parse_files(folder_files).await);
            } else {
                debug!("File detected, parsing file");
                let parsed_file: Vec<T> = parse(file).await.unwrap();
                parsed_files.extend(parsed_file);
            }
        }
        parsed_files
    })
    .await
}

/// Given a list of files and folders, parse them into a dictionary of type `T`
/// where the key is the filename (without the extension) and the value is the parsed data structure.
///
/// # Arguments
///
/// - `files`
///  A list of files and folders to parse
///
/// # Returns
///
/// A dictionary of parsed data structures of type `T`
#[tracing::instrument]
pub async fn parse_files_with_filename<T>(
    files: Vec<PathBuf>,
) -> std::collections::HashMap<String, Vec<T>>
where
    T: for<'de> serde::de::Deserialize<'de> + std::fmt::Debug,
{
    Box::pin(async move {
        let mut parsed_files: std::collections::HashMap<String, Vec<T>> =
            std::collections::HashMap::new();
        for file in files {
            debug!("Parsing file: {:?}", file);
            // If file is a folder, call this function recursively
            if file.is_dir() {
                debug!("Folder detected, parsing files in folder");
                let mut folder_files: Vec<PathBuf> = Vec::new();
                for entry in std::fs::read_dir(file).unwrap() {
                    let entry = entry.unwrap();
                    folder_files.push(entry.path());
                }
                let parsed_folder_files = parse_files_with_filename(folder_files).await;
                parsed_files.extend(parsed_folder_files);
            } else {
                debug!("File detected, parsing file");
                let filename = file.file_stem().unwrap().to_str().unwrap().to_string();
                let parsed_file: Vec<T> = parse(file).await.unwrap();
                parsed_files.insert(filename, parsed_file);
            }
        }
        parsed_files
    })
    .await
}
