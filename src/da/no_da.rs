pub struct NoDAConfig;

use async_trait::async_trait;

use crate::app::config::AppChainConfig;
use crate::da::da_layers::{DaClient, DaError};

#[async_trait]
impl DaClient for NoDAConfig {
    fn generate_da_config(&self, config: &AppChainConfig) -> Result<(), DaError> {
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
