use async_trait::async_trait;
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use thiserror::Error;

use crate::app::config::AppChainConfig;
use crate::da::avail::{AvailClient, AvailError};
use crate::da::ethereum::EthereumClient;
use crate::da::ethereum::EthereumError;
use crate::da::no_da::NoDAConfig;
use crate::utils::constants::APP_DA_CONFIG_NAME;
use crate::utils::paths::get_app_home;
use eyre::Result as EyreResult;

#[derive(Debug, Serialize, Deserialize, EnumIter, Display, Clone)]
pub enum DALayer {
    Avail,
    Ethereum,
    NoDA,
}

#[derive(Error, Debug)]
pub enum DaError {
    #[error("avail error: {0}")]
    AvailError(#[from] AvailError),
    #[error("ethereum error: {0}")]
    EthereumError(#[from] EthereumError),
    #[error("failed to read app home: {0}")]
    FailedToReadAppHome(io::Error),
    #[error("inquire error")]
    InquireError(#[from] inquire::InquireError),
    #[error("Failed to read DA config file")]
    FailedToReadDaConfigFile(io::Error),
    #[error("Failed to deserialize config")]
    FailedToDeserializeDaConfig(serde_json::Error),
    #[error("Failed to serialize config")]
    FailedToSerializeDaConfig(serde_json::Error),
    #[error("Failed to write DA config to file")]
    FailedToWriteDaConfigToFile(io::Error),
}

#[async_trait]
pub trait DaClient {
    fn setup_and_generate_keypair(&self, config: &AppChainConfig) -> eyre::Result<()>;

    fn confirm_minimum_balance(&self, config: &AppChainConfig) -> Result<(), DaError>;

    fn get_da_config_path(&self, config: &AppChainConfig) -> Result<PathBuf, DaError> {
        Ok(get_app_home(&config.app_chain).map_err(DaError::FailedToReadAppHome)?.join(APP_DA_CONFIG_NAME))
    }

    async fn setup(&self, config: &AppChainConfig) -> EyreResult<()>;
}

pub struct DAFactory;

impl DAFactory {
    pub fn new_da(da: &DALayer) -> Box<dyn DaClient> {
        match da {
            DALayer::Avail => Box::new(AvailClient {}),
            DALayer::Ethereum => Box::new(EthereumClient {}),
            _ => Box::new(NoDAConfig {}),
        }
    }
}
