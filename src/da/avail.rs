use std::fs;

use hex::encode;
use serde_json::json;
use sp_core::{sr25519, Pair};

use crate::da::da_layers::DaConfig;
use crate::da::errors::KeyGenError;
use crate::utils::paths::get_app_home;

pub struct AvailConfig {}

impl DaConfig for AvailConfig {
    fn setup_and_generate_keypair(&self, app_chain: &str) -> Result<(), KeyGenError> {
        let file_path = get_app_home(app_chain)?.join(format!("{}-phrase.txt", app_chain));
        let file_path_str = file_path.to_string_lossy().to_string();
        let (pair, phrase, seed) = <sr25519::Pair as Pair>::generate_with_phrase(None);
        let public_key = format!("0x{}", pair.public());
        let seed_str = format!("0x{}", encode(seed.as_ref()));

        if let Err(err) = fs::write(file_path, phrase) {
            panic!("Error writing to file: {}", err);
        } else {
            log::info!("Secret phrase stored in app home: {}", file_path_str);
            log::info!("Please fund {} with atleast 1 AVL (https://docs.availproject.org/about/faucet/)", public_key);
        }

        generate_config(app_chain, &seed_str)?;

        Ok(())
    }
}

fn generate_config(app_chain: &str, seed: &str) -> Result<(), KeyGenError> {
    let file_path = get_app_home(app_chain)?.join(format!("{}-avail-config.json", app_chain));

    let avail_config = json! ({
        "ws_provider": "wss://goldberg.avail.tools:443/ws",
        "mode": "sovereign",
        "seed": seed,
        "app_id": 0,
    })
    .to_string();

    if let Err(err) = fs::write(file_path, avail_config) {
        panic!("Error writing to file: {}", err);
    } else {
        log::info!("Successfully generated Avail config!");
    }

    Ok(())
}
