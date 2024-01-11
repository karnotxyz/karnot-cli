pub struct NoDAConfig;

use crate::app::config::AppChainConfig;
use crate::da::da_layers::{DaClient, DaError};
use async_trait::async_trait;

#[async_trait]
impl DaClient for NoDAConfig {
    fn setup_and_generate_keypair(&self, config: &AppChainConfig) -> Result<(), DaError> {
        log::info!("Launching {} without any DA mode", config.app_chain);
        Ok(())
    }

    fn confirm_minimum_balance(&self, _config: &AppChainConfig) -> Result<(), DaError> {
        Ok(())
    }

    async fn setup(&self, _config: &AppChainConfig) -> eyre::Result<()> {
        Ok(())
    }
}
