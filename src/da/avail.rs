use std::fs;
use std::process::Stdio;

use serde_json::{json, Value};

use crate::da::da::DALayer;
use crate::da::errors::KeyGenError;
use crate::utils::cmd::{execute_cmd, execute_cmd_stdout};
use crate::utils::paths::{get_app_home, get_madara_home};

pub fn setup_and_generate_avail_keypair(app_chain: &str) -> Result<DALayer, KeyGenError> {
    install_subkey()?;
    let avail = generate_keypair(app_chain)?;

    Ok(avail)
}

fn install_subkey() -> Result<(), KeyGenError> {
    let madara_home = get_madara_home()?;
    execute_cmd("cargo", &["install", "subkey"], &madara_home)?;

    Ok(())
}
fn generate_keypair(app_chain: &str) -> Result<DALayer, KeyGenError> {
    let madara_home = get_madara_home()?;
    let full_file_path = get_app_home(app_chain)?.join(format!("{}-keypair.json", app_chain));

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

fn generate_avail_connect_config(app_chain: &str, seed: &str) -> Result<(), KeyGenError> {
    let full_file_path = get_app_home(app_chain)?.join(format!("{}-avail-connect.json", app_chain));

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
