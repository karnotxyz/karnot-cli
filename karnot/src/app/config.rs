use serde::{Deserialize, Serialize};
use strum::EnumIter;
use toml::ser::Error;

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

#[derive(Debug, Serialize, Deserialize, EnumIter, strum_macros::Display)]
pub enum RollupMode {
    Sovereign,
    // Validity,
    // Validium,
}

#[derive(Debug, Serialize, Deserialize, EnumIter, strum_macros::Display)]
pub enum DALayer {
    Avail,
    Celestia,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigVersion {
    Version1,
}
