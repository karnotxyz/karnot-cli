use crate::cli::constants::{MADARA_REPO_NAME, MADARA_REPO_ORG};
use crate::utils::cmd::execute_cmd;
use crate::utils::errors::MadaraError;
use crate::utils::github::git_clone;
use crate::utils::paths::get_madara_home;
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

    let base_path = format!("--base-path={}", config.base_path);
    execute_cmd("cargo", &["run", "--release", "setup", "--chain=dev", "--from-remote", &base_path], &madara_path)?;
    execute_cmd(
        "./target/release/madara",
        &["--rpc-cors=all", "--chain=dev", "--force-authoring", "--rpc-external", "--rpc-methods=unsafe", &base_path],
        &madara_path,
    )?;

    Ok(())
}
