use std::io;
use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyGenError {
    #[error("Failed to read file: {0}")]
    FailedToIoFilesystem(#[from] io::Error),
    #[error("Failed to parse output: {0}")]
    FailedToParseOutput(#[from] FromUtf8Error),
    #[error("Failed to parse to json: {0}")]
    FailedToParseToJson(#[from] serde_json::Error),
}
