pub struct NoDAConfig;

use crate::da::da_layers::DaConfig;
use crate::da::errors::KeyGenError;

impl DaConfig for NoDAConfig {
    fn setup_and_generate_keypair(&self, app_chain: &str) -> Result<(), KeyGenError> {
        log::info!("Selected DA in {} unavailable, falling to NoDA", app_chain);
        Ok(())
    }
}
