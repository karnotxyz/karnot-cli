use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

use crate::da::avail::AvailConfig;
use crate::da::errors::KeyGenError;
use crate::da::no_da::NoDAConfig;

#[derive(Debug, Serialize, Deserialize, EnumIter, Display)]
pub enum DALayer {
    Avail,
    Celestia,
    Ethereum,
    NoDA,
}

pub trait DaConfig {
    fn setup_and_generate_keypair(&self, app_chain: &str) -> Result<(), KeyGenError>;
}

pub struct DAFactory;

impl DAFactory {
    pub fn new_da(da: &DALayer) -> Box<dyn DaConfig> {
        match da {
            DALayer::Avail => Box::new(AvailConfig {}),
            _ => Box::new(NoDAConfig {}),
        }
    }
}
