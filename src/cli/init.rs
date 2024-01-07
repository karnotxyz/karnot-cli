use std::{fs, io};

use inquire::InquireError;
use strum::IntoEnumIterator;
use thiserror::Error;

use super::prompt::{get_option, get_text_input};
use crate::app::config::{AppChainConfig, ConfigVersion, RollupMode};
use crate::da::da_layers::{DAFactory, DALayer};
use crate::utils::constants::{APP_CONFIG_NAME, MADARA_REPO_NAME, MADARA_REPO_ORG};
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

pub fn init() {
    let config = match generate_config() {
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
    // fund_msg(&config.da_layer);
    log::info!("✅ New app chain initialised.");
}

fn generate_config() -> Result<AppChainConfig, InitError> {
    let app_chain = get_text_input("Enter you app chain name:", Some("madara"))?;

    let app_chains_home = get_app_chains_home()?;
    let binding = app_chains_home.join(format!("{}/data", app_chain));
    let default_base_path = binding.to_str().unwrap_or("madara-data");

    let base_path = get_text_input("Enter base path for data directory of your app chain:", Some(default_base_path))?;
    // let chain_id = get_text_input("Enter chain id for your app chain:", Some("MADARA"))?;
    let mode = get_option("Select mode for your app chain:", RollupMode::iter().collect::<Vec<_>>())?;
    let da_layer = get_option("Select DA layer for your app chain:", DALayer::iter().collect::<Vec<_>>())?;
    // let block_time =
    //     get_custom_input::<u64>("Enter block time of chain:", Some(1000), Some("Time in ms (e.g,
    // 1000, 2000)."))?; let disable_fees = get_boolean_input("Do you want to disable fees for your
    // app chain:", Some(false))?; let fee_token = get_text_input("Enter fee token:",
    // Some("STRK"))?;
    let madara_version = get_latest_commit_hash(MADARA_REPO_ORG, MADARA_REPO_NAME)?;
    let config_version = ConfigVersion::Version1;

    match DAFactory::new_da(&da_layer).setup_and_generate_keypair(&app_chain) {
        Ok(_) => (),
        Err(err) => {
            log::error!("Failed to generate keypair: {}", err);
            return Err(InitError::FailedToGenerateKeypair);
        }
    };

    Ok(AppChainConfig {
        app_chain,
        base_path,
        // chain_id,
        mode,
        da_layer,
        // block_time,
        // disable_fees,
        // fee_token,
        madara_version,
        config_version,
    })
}

fn write_config(config: &AppChainConfig) -> Result<(), InitError> {
    let toml = config.to_toml()?;
    let file_path = get_app_home(&config.app_chain)?.join(APP_CONFIG_NAME);

    if let Err(err) = fs::write(file_path, toml) {
        panic!("Error writing to file: {}", err);
    } else {
        log::info!("Config file saved!");
    }

    Ok(())
}
