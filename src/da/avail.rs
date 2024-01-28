use std::fs;

use async_trait::async_trait;
use hex::encode;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair};
use eyre::Result as EyreResult;
use thiserror::Error;

use crate::app::config::AppChainConfig;
use crate::cli::prompt::get_boolean_input;
use crate::da::da_layers::{DaClient, DaError};

pub struct AvailClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailConfig {
    pub ws_provider: String,
    pub mode: String,
    pub seed: String,
    pub app_id: u32,
    pub address: String,
}

#[derive(Error, Debug)]
pub enum AvailError {
    #[error("Failed to serialize config: {0}")]
    FailedToSerializeConfig(#[from] serde_json::Error),
    #[error("Faucet funds needed for DA to be submitted")]
    FaucetFundsNeeded,
}

const AVAIL_DOCS: &str = "https://docs.availproject.org/about/faucet/";

#[async_trait]
impl DaClient for AvailClient {
    async fn generate_da_config(&self, config: &AppChainConfig) -> EyreResult<()> {
        let file_path = self.get_da_config_path(config)?;
        let file_path_str = file_path.to_string_lossy().to_string();
        let (pair, phrase, seed) = <sr25519::Pair as Pair>::generate_with_phrase(None);
        let seed_str = format!("0x{}", encode(seed.as_ref()));

        if let Err(err) = fs::write(file_path, phrase) {
            panic!("Error writing to file: {}", err);
        } else {
            log::info!("🔑 Secret phrase stored in app home: {}", file_path_str);
            log::info!("💧 Avail address: {}", pair.public());
            log::info!(
                "📕 Please fund your Avail address to be able to submit blobs to the goldberg network. Docs: {}.",
                AVAIL_DOCS
            );
            log::info!(
                "🛠️ If you want to use a custom Avail address, please enter your seed and address in the config file: {}.",
                file_path_str
            );
        }

        write_config(file_path_str.as_str(), &seed_str, pair.public().to_string().as_str())?;

        Ok(())
    }

    fn confirm_minimum_balance(&self, config: &AppChainConfig) -> Result<(), DaError> {
        let avail_config_path = self.get_da_config_path(config)?;
        let avail_config: AvailConfig = serde_json::from_str(
            fs::read_to_string(avail_config_path).map_err(DaError::FailedToReadDaConfigFile)?.as_str(),
        )
        .map_err(DaError::FailedToDeserializeDaConfig)?;
        match get_boolean_input(
            format!(
                "Have you funded your Avail address {} using the faucet? Docs: {}",
                avail_config.address, AVAIL_DOCS
            )
            .as_str(),
            Some(true),
        )? {
            true => Ok(()),
            false => Err(DaError::AvailError(AvailError::FaucetFundsNeeded)),
        }
    }

    async fn setup(&self, _config: &AppChainConfig) -> eyre::Result<()> {
        Ok(())
    }
}

fn write_config(da_config_path: &str, seed: &str, address: &str) -> Result<(), DaError> {
    let avail_config = AvailConfig {
        ws_provider: "wss://goldberg.avail.tools:443/ws".to_string(),
        mode: "sovereign".to_string(),
        seed: seed.to_string(),
        app_id: 0,
        address: address.to_string(),
    };

    fs::write(da_config_path, serde_json::to_string(&avail_config).map_err(DaError::FailedToSerializeDaConfig)?)
        .map_err(DaError::FailedToWriteDaConfigToFile)?;

    Ok(())
}
