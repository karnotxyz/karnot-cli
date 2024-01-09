pub struct NoDAConfig;

use crate::app::config::AppChainConfig;
use crate::da::da_layers::{DaClient, DaError};

impl DaClient for NoDAConfig {
    fn setup_and_generate_keypair(&self, config: &AppChainConfig) -> Result<(), DaError> {
        log::info!("Launching {} without any DA mode", config.app_chain);
        Ok(())
    }

    fn confirm_minimum_balance(&self, config: &AppChainConfig) -> Result<(), DaError> {
        Ok(())
    }
}
