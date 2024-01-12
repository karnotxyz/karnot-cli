use crate::app::config::AppChainConfig;
use crate::da::da_layers::{DaClient, DaError};
use crate::utils::serde::bytes_from_hex_str;
use async_trait::async_trait;
use eyre::Result as EyreResult;

use ethers::contract::abigen;

use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer, WalletError};

use serde::{Deserialize, Serialize};
use std::fs;

use crate::cli::prompt::get_boolean_input;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;

pub struct EthereumClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumConfig {
    pub http_provider: String,
    pub core_contracts: String,
    pub sequencer_key: String,
    pub chain_id: u32,
    pub mode: String,
    pub poll_interval_ms: u32,
}

#[derive(Error, Debug)]
pub enum EthereumError {
    #[error("Failed to create wallet: {0}")]
    FailedToCreateWallet(WalletError),
    #[error("Failed to setup Starknet on Anvil")]
    FailedToSetupStarknet,
    #[error("Anvil node not running")]
    AnvilNodeNotRunning,
}

const ANVIL_DOCS: &str = "https://github.com/foundry-rs/foundry/tree/master/crates/anvil";

#[async_trait]
impl DaClient for EthereumClient {
    fn setup_and_generate_keypair(&self, config: &AppChainConfig) -> Result<(), DaError> {
        let file_path = self.get_da_config_path(config)?;
        let file_path_str = file_path.to_string_lossy().to_string();

        // TODO: generate a new random key for every new app chain
        let ethereum_config = EthereumConfig {
            http_provider: "http://localhost:8545".to_string(),
            core_contracts: "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512".to_string(),
            // default anvil key
            sequencer_key: "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
            chain_id: 31337,
            mode: "sovereign".to_string(),
            poll_interval_ms: 10,
        };

        fs::write(file_path_str, serde_json::to_string(&ethereum_config).map_err(DaError::FailedToSerializeDaConfig)?)
            .map_err(DaError::FailedToWriteDaConfigToFile)?;

        Ok(())
    }

    fn confirm_minimum_balance(&self, _config: &AppChainConfig) -> Result<(), DaError> {
        Ok(())
    }

    async fn setup(&self, config: &AppChainConfig) -> EyreResult<()> {
        match get_boolean_input(
            format!("Are you running an Anvil node locally? The CLI tool has been tested on Anvil version 0.2.0 (c312c0d). Docs: {}", ANVIL_DOCS).as_str(),
            Some(true),
        )? {
            true => Ok(()),
            false => Err(DaError::EthereumError(EthereumError::AnvilNodeNotRunning)),
        }?;

        let ethereum_config_path = self.get_da_config_path(config)?;
        let ethereum_config: EthereumConfig = serde_json::from_str(
            fs::read_to_string(ethereum_config_path).map_err(DaError::FailedToReadDaConfigFile)?.as_str(),
        )
        .map_err(DaError::FailedToDeserializeDaConfig)?;

        // get wallet
        let wallet =
            LocalWallet::from_str(&ethereum_config.sequencer_key).map_err(EthereumError::FailedToCreateWallet)?;

        // connect to the network
        let provider = Provider::<Http>::try_from(ethereum_config.http_provider.as_str())
            .map_err(|_| EthereumError::FailedToSetupStarknet)?
            .interval(Duration::from_millis(10u64));

        // instantiate the client with the wallet
        let client = Arc::new(SignerMiddleware::new(provider, wallet.clone().with_chain_id(ethereum_config.chain_id)));

        // deploye Starknet core contract
        abigen!(Starknet, "src/assets/Starknet.json");
        let starknet_contract = Starknet::deploy(client.clone(), ())?.send().await?;

        abigen!(UnsafeProxy, "src/assets/UnsafeProxy.json");
        let proxy_contract = UnsafeProxy::deploy(client.clone(), starknet_contract.address())?.send().await?;

        abigen!(
            StarknetInitializer,
            r#"[
                function initialize(bytes calldata data) external
                function registerOperator(address newOperator) external
            ]"#,
        );
        let initializer = StarknetInitializer::new(proxy_contract.address(), client);

        let mut bytes = [0u8; 7 * 32];
        bytes[32..64].copy_from_slice(
            bytes_from_hex_str::<32, true>("0x41fc2a467ef8649580631912517edcab7674173f1dbfa2e9b64fbcd82bc4d79")?
                .as_slice(),
        );
        bytes[96..128].copy_from_slice(
            bytes_from_hex_str::<32, true>("0x036f5e4ea4dd042801c8841e3db8e654124305da0f11824fc1db60c405dbb39f")?
                .as_slice(),
        );

        // 1. Provide Starknet OS program/config and genesis state
        initializer.initialize(bytes.into()).send().await?.await?;

        // 2. Add our EOA as Starknet operator
        initializer.register_operator(wallet.address()).send().await?.await?;

        Ok(())
    }
}
