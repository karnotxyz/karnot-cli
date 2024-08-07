use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum_macros::{Display, EnumString};
use toml::ser::Error;

use crate::da::da_layers::DALayer;

#[derive(Serialize, Deserialize)]
pub struct AppChainConfig {
    pub app_chain: String,
    pub base_path: String,
    pub mode: RollupMode,
    pub da_layer: DALayer,
    /// Stores commit hash of madara app chain build
    pub madara_version: String,
    /// Maintains version of config, which will help in handling edge
    /// cases when attributes are added / removed from struct
    pub config_version: ConfigVersion,
}

impl AppChainConfig {
    pub fn to_toml(&self) -> Result<String, Error> {
        toml::to_string(self)
    }
}

#[derive(Debug, Serialize, Deserialize, EnumIter, Display, Clone, EnumString, clap::ValueEnum)]
pub enum RollupMode {
    Sovereign,
    // Validity,
    // Validium,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigVersion {
    Version1,
    Version2,
}
