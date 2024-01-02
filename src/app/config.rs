use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum_macros::Display;
use toml::ser::Error;

use crate::da::da::DALayer;

#[derive(Serialize, Deserialize)]
pub struct AppChainConfig {
    pub app_chain: String,
    pub base_path: String,
    pub chain_id: String,
    pub mode: RollupMode,
    pub da_layer: DALayer,
    pub block_time: u64,
    pub disable_fees: bool,
    pub fee_token: String,
    /// Stores commit hash of madara app chain build
    pub madara_version: String,
    /// Maintains version of config, will help in handling edge
    /// cases when attributes are added / removed from struct
    pub config_version: ConfigVersion,
}

impl AppChainConfig {
    pub fn to_toml(&self) -> Result<String, Error> {
        toml::to_string(self)
    }
}

impl Default for AppChainConfig {
    fn default() -> Self {
        AppChainConfig {
            app_chain: "".to_string(),
            base_path: "".to_string(),
            chain_id: "".to_string(),
            mode: RollupMode::Sovereign,
            da_layer: DALayer::NoDA,
            block_time: 0,
            disable_fees: false,
            fee_token: "".to_string(),
            madara_version: "".to_string(),
            config_version: ConfigVersion::Version1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, EnumIter, Display)]
pub enum RollupMode {
    Sovereign,
    // Validity,
    // Validium,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigVersion {
    Version1,
}
