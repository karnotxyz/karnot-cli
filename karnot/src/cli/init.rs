use std::fs;
use std::path::PathBuf;

use crate as karnot;

use super::prompt::{
    ask_for_app_chain_name, ask_for_base_path, ask_for_block_time, ask_for_chain_id,
    ask_for_da_layer, ask_for_disable_fees, ask_for_fee_token, ask_for_mode,
};
use karnot::app::config::{AppChainConfig, ConfigVersion};

use serde::Deserialize;
use crate::app::utils::{get_app_home, get_app_chains_home};

pub fn init() {
    generate_config();
    println!("✅ New app chain initialised.");
}

fn generate_config() {
    let app_chain = ask_for_app_chain_name().unwrap();
    let base_path = ask_for_base_path().unwrap();
    let chain_id = ask_for_chain_id().unwrap();
    let mode = ask_for_mode().unwrap();
    let da_layer = ask_for_da_layer().unwrap();
    let block_time = ask_for_block_time().unwrap();
    let disable_fees = ask_for_disable_fees().unwrap();
    let fee_token = ask_for_fee_token().unwrap();
    let madara_version = get_latest_madara_commit_hash();
    let config_version = ConfigVersion::Version1;

    let config = AppChainConfig {
        app_chain,
        base_path,
        chain_id,
        mode,
        da_layer,
        block_time,
        disable_fees,
        fee_token,
        madara_version,
        config_version,
    };

    write_config(&config);
}

fn write_config(config: &AppChainConfig) {
    let toml = config.to_toml().unwrap();
    let config_file = format!("{}-config.toml", config.app_chain);
    let app_home = get_app_home(&config.app_chain).unwrap();
    let full_file_path= app_home.join(config_file);

    if let Err(err) = fs::write(&full_file_path, toml) {
        eprintln!("Error writing to file: {}", err);
    } else {
        println!("Data written to file successfully!");
    }
}

#[derive(Debug, Deserialize)]
struct Commit {
    sha: String,
}

fn get_latest_madara_commit_hash() -> String {
    let repo_owner = "keep-starknet-strange";
    let repo_name = "madara";
    let github_api_url = format!(
        "https://api.github.com/repos/{}/{}/commits",
        repo_owner, repo_name
    );

    let mut hash = String::new();

    // Init Tokio runtime
    let tokio_rt = tokio::runtime::Runtime::new().expect("Unable to create Tokio runtime");
    tokio_rt.block_on(async {
        match reqwest::get(&github_api_url).await {
            Ok(response) => {
                if let Ok(commits) = response.json::<Vec<Commit>>().await {
                    if let Some(latest_commit) = commits.first() {
                        hash = latest_commit.sha.clone();
                    }
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    });

    hash
}
