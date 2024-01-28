use std::{fs, io};

use inquire::InquireError;
use strum::IntoEnumIterator;
use thiserror::Error;

use super::prompt::{get_option, get_text_input};
use crate::app::config::{AppChainConfig, ConfigVersion, RollupMode};
use crate::da::da_layers::{DAFactory, DALayer};
use crate::utils::constants::{APP_CONFIG_NAME, MADARA_BRANCH_NAME, MADARA_REPO_NAME, MADARA_REPO_ORG};
use crate::utils::errors::GithubError;
use crate::utils::github::get_latest_commit_hash;
use crate::utils::paths::{get_app_chains_home, get_app_home};

#[derive(Debug, Error)]
pub enum InitError {
    #[error("Failed to get input: {0}")]
    FailedToGetInout(#[from] InquireError),
    #[error("Failed to write config: {0}")]
    FailedToWriteConfig(#[from] io::Error),
    #[error("Failed to get latest commit hash: {0}")]
    FailedToGetLatestCommitHash(#[from] GithubError),
    #[error("Failed to serialize to toml: {0}")]
    FailedToSerializeToToml(#[from] toml::ser::Error),
    #[error("Failed to generate keypair")]
    FailedToGenerateKeypair,
}

pub async fn init() {
    let config = match generate_config().await {
        Ok(config) => config,
        Err(err) => {
            panic!("Failed to get input: {}", err);
        }
    };
    match write_config(&config) {
        Ok(config) => config,
        Err(err) => {
            panic!("Failed to write config: {}", err);
        }
    };

    log::info!("\n");
    log::info!("✅ New app chain initialised.");
}

async fn generate_config() -> Result<AppChainConfig, InitError> {
    let app_chain = get_text_input("Enter you app chain name:", Some("madara"))?;

    let app_chains_home = get_app_chains_home()?;
    let binding = app_chains_home.join(format!("{}/data", app_chain));
    let default_base_path = binding.to_str().unwrap_or("madara-data");

    let mode = get_option("Select mode for your app chain:", RollupMode::iter().collect::<Vec<_>>())?;
    let da_layer = get_option("Select DA layer for your app chain:", DALayer::iter().collect::<Vec<_>>())?;
    let madara_version = get_latest_commit_hash(MADARA_REPO_ORG, MADARA_REPO_NAME, MADARA_BRANCH_NAME).await?;
    let config_version = ConfigVersion::Version2;

    log::info!("\n");

    let config = AppChainConfig {
        app_chain,
        base_path: default_base_path.to_string(),
        mode,
        da_layer: da_layer.clone(),
        madara_version,
        config_version,
    };

    match DAFactory::new_da(&da_layer).setup_and_generate_keypair(&config) {
        Ok(_) => (),
        Err(err) => {
            log::error!("Failed to generate keypair: {}", err);
            return Err(InitError::FailedToGenerateKeypair);
        }
    };

    Ok(config)
}

fn write_config(config: &AppChainConfig) -> Result<(), InitError> {
    let toml = config.to_toml()?;
    let file_path = get_app_home(&config.app_chain)?.join(APP_CONFIG_NAME);

    if let Err(err) = fs::write(file_path, toml) {
        panic!("Error writing to file: {}", err);
    } else {
        log::debug!("Config file saved!");
    }

    Ok(())
}
