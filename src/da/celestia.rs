use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

use async_trait::async_trait;
use bollard::models::{HostConfig, Mount, PortBinding};
use serde::{Deserialize, Serialize};
use eyre::Result as EyreResult;
use eyre::Report as EyreReport;
use thiserror::Error;

use crate::app::config::AppChainConfig;
use crate::cli::prompt::get_boolean_input;
use crate::da::da_layers::{DaClient, DaError};
use crate::utils::docker::{container_exists, is_container_running, kill_container, run_docker_image};
use crate::utils::paths::{get_madara_home};
use std::time::Duration;

pub struct CelestiaClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct CelestiaConfig {
    pub ws_provides: String,
    pub http_provider: String,
    pub nid: String,
    pub auth_token: String,
    pub address: String,
}

#[derive(Error, Debug)]
pub enum CelestiaError {
    #[error("Faucet funds needed for DA to be submitted")]
    FaucetFundsNeeded,
    #[error("Celestia light node setup failed")]
    SetupError,
    #[error("Failed to read celestia home")]
    FailedToReadCelestiaHome,
    #[error("Failed to run in celestia container")]
    FailedToRunInCelestiaContainer,
}

const CELESTIA_DOCS: &str = "https://docs.celestia.org/developers/celestia-app-wallet#fund-a-wallet";
const CELESTIA_CONTAINER_NAME: &str = "celestia-light-client";

#[async_trait]
impl DaClient for CelestiaClient {
    async fn generate_da_config(&self, config: &AppChainConfig) -> EyreResult<()> {
        let celestia_home = get_celestia_home()?;
        let file_keys_txt = celestia_home.join("keys.txt");
        let file_auth_txt = celestia_home.join("auth.txt");

        if !file_keys_txt.exists() ||  !file_auth_txt.exists() {
            let run_cmd = vec![
                "sh",
                "-c",
                "celestia light init --p2p.network=mocha > /home/celestia/keys.txt &&\
                 celestia light auth admin --p2p.network=mocha > /home/celestia/auth.txt"
            ];
            exec_cmd_in_celestia_container(run_cmd).await?;
            // Waits for docker container to execute the commands and generate the keys
            loop {
                let container_exists = is_container_running(&CELESTIA_CONTAINER_NAME).await;
                if !container_exists {
                    break; // Container has exited
                }

                // Sleep for a brief period to avoid excessive polling
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }

        let file_path = self.get_da_config_path(config)?;
        let file_path_str = file_path.to_string_lossy().to_string();

        let keys_txt_content = fs::read_to_string(file_keys_txt)?;
        let auth_token = fs::read_to_string(file_auth_txt)?;

        let mut address: &str = "";
        for line in keys_txt_content.lines() {
            if line.trim().starts_with("ADDRESS:") {
                address = line.trim_start_matches("ADDRESS:").trim();
                log::info!("🔑 Secret phrase stored in app home: {}", celestia_home.to_string_lossy().to_string());
                log::info!("💧 Celestia address: {}", address);
                log::info!(
                    "=> Please fund your Celestia address to be able to submit blobs to the mocha network. Docs: {}",
                    CELESTIA_DOCS
                )
            }
        }

        if address.is_empty()|| auth_token.is_empty() {
            return Err(EyreReport::from(DaError::CelestiaError(CelestiaError::SetupError)));
        }

        write_config(file_path_str.as_str(), auth_token.trim(), address)?;

        Ok(())
    }

    fn confirm_minimum_balance(&self, config: &AppChainConfig) -> Result<(), DaError> {
        let celestia_config_path = self.get_da_config_path(config)?;
        let celestia_config: CelestiaConfig = serde_json::from_str(
            fs::read_to_string(celestia_config_path).map_err(DaError::FailedToReadDaConfigFile)?.as_str(),
        )
        .map_err(DaError::FailedToDeserializeDaConfig)?;
        match get_boolean_input(
            format!(
                "Have you funded your Celestia address {} using the faucet? Docs: {}",
                celestia_config.address, CELESTIA_DOCS
            )
            .as_str(),
            Some(true),
        )? {
            true => Ok(()),
            false => Err(DaError::CelestiaError(CelestiaError::FaucetFundsNeeded)),
        }
    }

    async fn setup(&self, _config: &AppChainConfig) -> eyre::Result<()> {
        let run_cmd = vec![
            "sh",
            "-c",
            "celestia light start --core.ip=rpc-mocha.pops.one --p2p.network=mocha",
        ];
        exec_cmd_in_celestia_container(run_cmd).await
    }
}

pub async fn exec_cmd_in_celestia_container(run_cmd: Vec<&str>) -> EyreResult<()> {
    let celestia_home = get_celestia_home()?;
    let celestia_home_str = celestia_home.to_str().unwrap_or("~/.madara/celestia");

    let env = vec!["NODE_TYPE=light", "P2P_NETWORK=mocha"];

    let mut port_bindings = HashMap::new();
    port_bindings.insert(
        "26658/tcp".to_string(),
        Some(vec![PortBinding { host_ip: Some("0.0.0.0".to_string()), host_port: Some("26658".to_string()) }]),
    );

    let host_config = HostConfig {
        mounts: Some(vec![Mount {
            target: Some("/home/celestia".to_string()),
            source: Some(celestia_home_str.to_string()),
            typ: Some(bollard::models::MountTypeEnum::BIND),
            ..Default::default()
        }]),
        port_bindings: Some(port_bindings),
        ..Default::default()
    };

    if container_exists(CELESTIA_CONTAINER_NAME).await {
        // TODO: handle error
        let _ = kill_container(CELESTIA_CONTAINER_NAME).await;
    }

    run_docker_image(
        "ghcr.io/celestiaorg/celestia-node:v0.12.2",
        CELESTIA_CONTAINER_NAME,
        Some(env),
        Some(host_config),
        Some(run_cmd),
    )
    .await;
    log::info!("🧭 Command ran on Celestia light client\n");

    Ok(())
}

fn write_config(da_config_path: &str, auth_token: &str, address: &str) -> Result<(), DaError> {
    let celestia_config = CelestiaConfig {
        ws_provides: "http://127.0.0.1:26658".to_string(),
        http_provider: "http://127.0.0.1:26658".to_string(),
        nid: "Madara".to_string(),
        auth_token: auth_token.to_string(),
        address: address.to_string(),
    };

    fs::write(da_config_path, serde_json::to_string(&celestia_config).map_err(DaError::FailedToSerializeDaConfig)?)
        .map_err(DaError::FailedToWriteDaConfigToFile)?;

    Ok(())
}

pub fn get_celestia_home() -> Result<PathBuf, Error> {
    let madara_home = get_madara_home()?;
    let celestia_home = madara_home.join("celestia");

    // Creates the `celestia` directory if not present
    fs::create_dir_all(&celestia_home)?;

    Ok(celestia_home)
}
