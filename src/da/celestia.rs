use std::collections::HashMap;
use std::fs;

use async_trait::async_trait;
use bollard::models::{HostConfig, Mount, PortBinding};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::app::config::AppChainConfig;
use crate::cli::prompt::get_boolean_input;
use crate::da::da_layers::{DaClient, DaError};
use crate::utils::docker::{container_exists, kill_container, run_docker_image};
use crate::utils::paths::get_celestia_home;
use std::thread;
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
}

const CELESTIA_DOCS: &str = "https://docs.celestia.org/developers/celestia-app-wallet#fund-a-wallet";

#[async_trait]
impl DaClient for CelestiaClient {
    async fn generate_da_config(&self, config: &AppChainConfig) -> Result<(), DaError> {
        let celestia_home = get_celestia_home().unwrap();
        let file_keys_txt = celestia_home.join("keys.txt");
        let file_auth_txt = celestia_home.join("auth.txt");

        if !file_keys_txt.exists() || !file_auth_txt.exists() {
            let run_cmd = vec![
                "sh",
                "-c",
                "celestia light init --p2p.network=mocha > /home/celestia/keys.txt &&\
                 celestia light auth admin --p2p.network=mocha > /home/celestia/auth.txt"
            ];
            run_celestia_light_node(run_cmd).await.unwrap();
            // Waits for docker container to execute the commands and generate the keys
            thread::sleep(Duration::from_secs(5));
        }

        let file_path = self.get_da_config_path(config)?;
        let file_path_str = file_path.to_string_lossy().to_string();

        let keys_txt_content = match fs::read_to_string(file_keys_txt) {
            Ok(content) => content,
            Err(_) => {
                return Err(DaError::CelestiaError(CelestiaError::SetupError));
            }
        };

        let auth_token = match fs::read_to_string(file_auth_txt) {
            Ok(content) => content,
            Err(_) => {
                return Err(DaError::CelestiaError(CelestiaError::SetupError));
            }
        };

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

        if address.eq("") || auth_token.eq("") {
            return Err(DaError::CelestiaError(CelestiaError::SetupError));
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
        run_celestia_light_node(run_cmd).await
    }
}

pub async fn run_celestia_light_node(run_cmd: Vec<&str>) -> eyre::Result<()> {
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

    // let start_cmd = vec![
    //     "sh",
    //     "-c",
    //     "celestia light init --p2p.network=mocha > /home/celestia/keys.txt &&celestia light auth admin \
    //      --p2p.network=mocha > /home/celestia/auth.txt &&celestia light start --core.ip=rpc-mocha.pops.one \
    //      --p2p.network=mocha",
    // ];

    const CONTAINER_NAME: &str = "celestia-light-client";

    if container_exists(CONTAINER_NAME).await {
        // TODO: handle error
        let _ = kill_container(CONTAINER_NAME).await;
    }

    run_docker_image(
        "ghcr.io/celestiaorg/celestia-node:v0.12.2",
        CONTAINER_NAME,
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
