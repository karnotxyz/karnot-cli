use crate::app::config::AppChainConfig;
use crate::da::da_layers::{DaClient, DaError};
use crate::utils::serde::bytes_from_hex_str;
use async_trait::async_trait;
use eyre::Result as EyreResult;

use ethers::contract::abigen;

use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, MnemonicBuilder, Signer, WalletError};

use serde::{Deserialize, Serialize};
use std::fs;

use ethers::signers::coins_bip39::English;
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
}

const SEPOLIA_FAUCET_LINKS: &str = "https://faucetlink.to/sepolia";

#[async_trait]
impl DaClient for EthereumClient {
    fn setup_and_generate_keypair(&self, config: &AppChainConfig) -> eyre::Result<()> {
        let file_path = self.get_da_config_path(config)?;
        let file_path_str = file_path.to_string_lossy().to_string();

        let mut rng = rand::thread_rng();
        let wallet = MnemonicBuilder::<English>::default()
            .word_count(24)
            .derivation_path("m/44'/60'/0'/2/1")?
            .build_random(&mut rng)?;

        let ethereum_config = EthereumConfig {
            http_provider: "http://localhost:8545".to_string(),
            core_contracts: "".to_string(),
            // default anvil key
            sequencer_key: hex::encode(wallet.signer().to_bytes()),
            chain_id: 31337,
            mode: "sovereign".to_string(),
            poll_interval_ms: 10,
        };

        fs::write(
            file_path_str.clone(),
            serde_json::to_string(&ethereum_config).map_err(DaError::FailedToSerializeDaConfig)?,
        )
        .map_err(DaError::FailedToWriteDaConfigToFile)?;

        log::info!("🔑 Secret phrase stored in app home: {}", file_path_str);
        log::info!("💧 Ethereum address: {:?}", wallet.address());
        log::info!(
            "=> Please fund your Ethereum address to do the setup on the Sepolia network. Docs: {}",
            SEPOLIA_FAUCET_LINKS
        );

        Ok(())
    }

    fn confirm_minimum_balance(&self, _config: &AppChainConfig) -> Result<(), DaError> {
        Ok(())
    }

    async fn setup(&self, config: &AppChainConfig) -> EyreResult<()> {
        let ethereum_config_path = self.get_da_config_path(config)?;
        let mut ethereum_config: EthereumConfig = serde_json::from_str(
            fs::read_to_string(ethereum_config_path).map_err(DaError::FailedToReadDaConfigFile)?.as_str(),
        )
        .map_err(DaError::FailedToDeserializeDaConfig)?;

        if !ethereum_config.core_contracts.is_empty() {
            log::info!("✅ Ethereum contracts already deployed");
            return Ok(());
        }

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

        // overwrite Ethereum config with core contract address
        ethereum_config.core_contracts = proxy_contract.address().to_string();

        let file_path = self.get_da_config_path(config)?.to_string_lossy().to_string();
        fs::write(file_path, serde_json::to_string(&ethereum_config).map_err(DaError::FailedToSerializeDaConfig)?)
            .map_err(DaError::FailedToWriteDaConfigToFile)?;

        Ok(())
    }
}
