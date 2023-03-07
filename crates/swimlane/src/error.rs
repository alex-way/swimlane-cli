use reqwest::Error as ReqwestError;
use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
pub enum SwimlaneClientError {
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    #[error("Python Package Not Found {package:?} {version:?}")]
    PackageNotFound { package: String, version: String },
    #[error("File not found {0}")]
    FileNotFound(String),
    #[error("IO Error")]
    IoError(#[from] io::Error),
    #[error("Request Error")]
    ReqwestError(#[from] ReqwestError),
}

#[derive(Debug, Clone)]
pub struct InvalidFormat;

#[derive(Debug, Error)]
pub enum UploadRequirementsError {
    #[error("File not found")]
    FileNotFound(#[from] io::Error),
    // #[error("I/O error")]
    // Io(#[from] io::Error),
    #[error("File isn't in the correct format")]
    InvalidFormat { line_number: usize, line: String },
    #[error("Package has been specified twice: {key} on line {line_number} with value {existing_value} and {new_value}")]
    DuplicatePackage {
        key: String,
        line_number: usize,
        existing_value: String,
        new_value: String,
    },
}
