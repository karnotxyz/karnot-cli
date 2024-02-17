use crate::app::config::{AppChainConfig, ConfigVersion};
use crate::da::da_layers::DALayer;
use crate::utils::cmd::execute_cmd;
use crate::utils::constants::{APP_DA_CONFIG_NAME, MADARA_REPO_NAME, MADARA_REPO_ORG};
use crate::utils::errors::MadaraError;
use crate::utils::github::git_clone;
use crate::utils::paths::{get_app_home, get_madara_home};

pub const GITHUB_BASE_URL: &str = "https://github.com";

pub fn clone_madara_and_build_repo(config: &AppChainConfig) -> Result<(), MadaraError> {
    let repo_url = format!("{}/{}/{}", GITHUB_BASE_URL, MADARA_REPO_ORG, MADARA_REPO_NAME);
    let madara_path = get_madara_home()?.join("madara");

    // there was a bug in Version1 where the incorrect commit was
    // going inside the toml file, so override with the correct commit
    // in that case
    let checkout_commit = match config.config_version {
        ConfigVersion::Version2 => config.madara_version.as_str(),
        ConfigVersion::Version1 => "5de416aeb2d9e4297e58f7f2dff99aeae521855e",
    };

    match git_clone(&repo_url, &madara_path, Some(checkout_commit)) {
        Ok(_) => {
            log::info!("Successfully cloned Madara repo");
        }
        Err(err) => {
            log::error!("Failed to clone Madara repo: {}", err);
            return Err(MadaraError::FailedToCloneRepo);
        }
    }
    execute_cmd("cargo", &["build", "--release", "--features", "avail", "--features", "celestia"], &madara_path)?;

    Ok(())
}

pub fn setup_and_run_madara(config: AppChainConfig, flags: &String) -> Result<(), MadaraError> {
    let madara_path = get_madara_home()?.join("madara");

    let app_home = get_app_home(config.app_chain.as_str())?;
    let binding = app_home.join(APP_DA_CONFIG_NAME);
    let da_config_path = match binding.to_str() {
        Some(path) => path,
        None => {
            return Err(MadaraError::FailedToGetDAConfig);
        }
    };

    let da_conf = format!("--da-conf={}", da_config_path);
    let base_path = format!("--base-path={}", config.base_path);

    let mut args = vec![
        "--chain=dev",
        "--alice",
        "--force-authoring",
        "--rpc-cors=all",
        "--tx-ban-seconds=0",
        "--prometheus-external",
        "--rpc-external",
        &base_path,
    ];

    args.extend(
        flags
            .split(",")
            .map(|flag| flag.trim())
            .filter(|flag| !flag.is_empty())
    );

    match config.da_layer {
        DALayer::Ethereum => {
            let ethereum_conf = vec!["--da-layer=ethereum", &da_conf];
            args.extend(ethereum_conf);
        }
        DALayer::Avail => {
            let avail_conf = vec!["--da-layer=avail", &da_conf];
            args.extend(avail_conf);
        }
        _ => (),
    }

    let config_path =
        madara_path.join("configs").into_os_string().into_string().map_err(MadaraError::FailedToConvertToString)?;

    execute_cmd(
        "./target/release/madara",
        &["setup", "--chain=dev", "--from-local", config_path.as_str(), &base_path],
        &madara_path,
    )?;

    execute_cmd("./target/release/madara", args.as_slice(), &madara_path)?;

    Ok(())
}
