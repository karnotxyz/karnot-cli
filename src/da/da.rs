use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

use crate::da::avail::generate_avail_keypair;
use crate::da::errors::KeyGenError;

#[derive(Debug, Serialize, Deserialize, EnumIter, Display)]
pub enum DALayer {
    Avail { seed: String, public_key: String },
    Celestia,
    Ethereum,
    NoDA,
}

pub trait DaConfig {
    fn setup_and_generate_keypair(&self, app_chain: &str) -> Result<DALayer, KeyGenError>;
}

impl DaConfig for DALayer {
    fn setup_and_generate_keypair(&self, app_chain: &str) -> Result<DALayer, KeyGenError> {
        match self {
            DALayer::Avail { .. } => generate_avail_keypair(app_chain),
            _ => {
                log::warn!("DA layer implementation unavailable, falling back to NoDA");
                Ok(DALayer::NoDA)
            }
        }
    }
}
