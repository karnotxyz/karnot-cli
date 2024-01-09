use std::ffi::OsString;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GithubError {
    #[error("Failed to get commits from Github")]
    FailedToGetCommits(#[from] reqwest::Error),
    #[error("No commits found")]
    NoCommitsFound,
    #[error("Failed to clone Github repo")]
    FailedToCloneRepo,
    #[error("Unable to execute command")]
    CommandExecutionFailed(#[from] std::io::Error),
    #[error("Unable to fetch remote")]
    RemoteFetchFailed(#[from] git2::Error),
}

#[derive(Debug, Error)]
pub enum MadaraError {
    #[error("Failed to read the file: {0}")]
    FailedToReadFile(#[from] std::io::Error),
    #[error("Failed to parse toml file: {0}")]
    FailedToParseToml(#[from] toml::de::Error),
    #[error("Failed to clone repo")]
    FailedToCloneRepo,
    #[error("Failed to regenerate config")]
    FailedToRegenerateConfig,
    #[error("Failed to get DA config")]
    FailedToGetDAConfig,
    #[error("Unable to fetch remote")]
    FailedToConvertToString(OsString),
}

#[derive(Debug, Error)]
pub enum TomlError {
    #[error("Failed to read the file: {0}")]
    FailedToReadFile(#[from] std::io::Error),
    #[error("Failed to parse toml file: {0}")]
    FailedToParseToml(#[from] toml::de::Error),
}
