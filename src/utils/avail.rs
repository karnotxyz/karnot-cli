use std::process::Stdio;
use std::string::FromUtf8Error;
use std::{fs, io};

use serde_json::{json, Value};
use thiserror::Error;

use crate::app::config::DALayer;
use crate::utils::cmd::{execute_cmd, execute_cmd_stdout};
use crate::utils::paths::{get_app_home, get_madara_home};

#[derive(Debug, Error)]
pub enum AvailKeyGenError {
    #[error("Failed to read file: {0}")]
    FailedToIoFilesystem(#[from] io::Error),
    #[error("Failed to parse output: {0}")]
    FailedToParseOutput(#[from] FromUtf8Error),
    #[error("Failed to parse to json: {0}")]
    FailedToParseToJson(#[from] serde_json::Error),
}

pub fn setup_and_generate_keypair(app_chain: &str) -> Result<DALayer, AvailKeyGenError> {
    install_subkey()?;
    let avail = generate_keypair(app_chain)?;

    Ok(avail)
}
fn install_subkey() -> Result<(), AvailKeyGenError> {
    let madara_home = get_madara_home()?;
    execute_cmd("cargo", &["install", "subkey"], &madara_home)?;

    Ok(())
}
fn generate_keypair(app_chain: &str) -> Result<DALayer, AvailKeyGenError> {
    let madara_home = get_madara_home()?;
    let keypair_file = format!("{}-keypair.json", app_chain);
    let app_home = get_app_home(app_chain)?;
    let full_file_path = app_home.join(keypair_file);

    execute_cmd("cargo", &["install", "subkey"], &madara_home)?;

    let output = execute_cmd_stdout("subkey", &["generate", "--output-type=json"], &madara_home, Stdio::piped())?;

    let keypair_str = String::from_utf8(output.stdout)?;
    let keypair_json: Value = serde_json::from_str(&keypair_str)?;

    let seed = keypair_json["secretSeed"].to_string();
    let public_key = keypair_json["publicKey"].to_string();
    let avail = DALayer::Avail { seed: seed.clone(), public_key };

    if let Err(err) = fs::write(full_file_path, &keypair_str) {
        panic!("Error writing to file: {}", err);
    } else {
        log::info!("Data written to file successfully!");
    }

    generate_avail_connect_config(app_chain, &seed)?;

    Ok(avail)
}

fn generate_avail_connect_config(app_chain: &str, seed: &str) -> Result<(), AvailKeyGenError> {
    let connect_file = format!("{}-avail-connect.json", app_chain);
    let app_home = get_app_home(app_chain)?;
    let full_file_path = app_home.join(connect_file);

    let avail_connect = json! ({
        "ws_provider": "wss://goldberg.avail.tools:443/ws",
        "mode": "sovereign",
        "seed": seed,
        "app_id": 0,
    });

    let avail_connect_string = serde_json::to_string(&avail_connect)?;

    // "\"{seed}\"" is invalid so converted it to "{seed}"
    let avail_connect_string_modified = avail_connect_string.replace("\"\\\"", "\"").replace("\\\"\"", "\"");

    if let Err(err) = fs::write(full_file_path, &avail_connect_string_modified) {
        panic!("Error writing to file: {}", err);
    } else {
        log::info!("Data written to file successfully!");
    }

    Ok(())
}
