use crate::cli::constants::{MADARA_REPO_NAME, MADARA_REPO_ORG};
use crate::da::da::DALayer;
use crate::utils::cmd::execute_cmd;
use crate::utils::errors::MadaraError;
use crate::utils::github::git_clone;
use crate::utils::paths::{get_app_home, get_madara_home};
use crate::utils::toml::regenerate_app_config;
pub const GITHUB_BASE_URL: &str = "https://github.com";

pub fn clone_madara_and_build_repo() -> Result<(), MadaraError> {
    let repo_url = format!("{}/{}/{}", GITHUB_BASE_URL, MADARA_REPO_ORG, MADARA_REPO_NAME);
    let madara_path = get_madara_home()?.join("madara");

    match git_clone(&repo_url, &madara_path) {
        Ok(_) => {
            log::info!("Successfully cloned Madara repo");
        }
        Err(err) => {
            log::error!("Failed to clone Madara repo: {}", err);
            return Err(MadaraError::FailedToCloneRepo);
        }
    }
    execute_cmd("cargo", &["build", "--release"], &madara_path)?;

    Ok(())
}

pub fn setup_and_run_madara(app_chain: &str) -> Result<(), MadaraError> {
    let madara_path = get_madara_home()?.join("madara");

    let (config, _) = match regenerate_app_config(app_chain) {
        Ok((config, valid)) => (config, valid),
        Err(err) => {
            log::error!("Failed to fetch the required app chain: {}", err);
            return Err(MadaraError::FailedToRegenerateConfig);
        }
    };

    let app_home = get_app_home(app_chain)?;
    let binding = app_home.join(format!("{}-avail-connect.json", app_chain));
    let da_config_path = match binding.to_str() {
        Some(path) => path,
        None => {
            return Err(MadaraError::FailedToGetDAConfig);
        }
    };

    let da_conf = format!("--da-conf={}", da_config_path);
    let base_path = format!("--base-path={}", config.base_path);

    let mut args =
        vec!["--chain=dev", "--alice", "--force-authoring", "--rpc-cors=all", "--tx-ban-seconds=0", &base_path];

    match &config.da_layer {
        DALayer::Avail { .. } => {
            let avail_conf = vec!["--da-layer=avail", &da_conf];
            args.extend(avail_conf);
        }
        _ => {}
    }

    execute_cmd("cargo", &["run", "--release", "setup", "--chain=dev", "--from-remote", &base_path], &madara_path)?;

    execute_cmd("./target/release/madara", args.as_slice(), &madara_path)?;

    Ok(())
}
