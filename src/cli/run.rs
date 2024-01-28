use inquire::InquireError;
use thiserror::Error;

use crate::cli::list::get_apps_list;
use crate::cli::prompt::get_option;
use crate::da::da_layers::{DAFactory, DaError};
use crate::utils::errors::MadaraError;
use crate::utils::madara;
use crate::utils::toml::regenerate_app_config;

#[derive(Debug, Error)]
pub enum RunError {
    #[error("Failed to get input: {0}")]
    FailedToGetInout(#[from] InquireError),
    #[error("Failed to start madara: {0}")]
    FailedToStartMadara(#[from] MadaraError),
    #[error("Failed to get app chains: {0}")]
    FailedToGetAppChains(#[from] std::io::Error),
    #[error("Failed to regenerate config: {0}")]
    FailedToRegenerateConfig(String),
    #[error("Failed with DA error: {0}")]
    FailedWithDaError(#[from] DaError),
    #[error(transparent)]
    Other(#[from] eyre::Error),
}

pub async fn run() {
    match start_app_chain().await {
        Ok(_) => {
            log::info!("Madara setup successful");
        }
        Err(err) => {
            log::error!("Failed to setup Madara: {}", err);
        }
    }
}

async fn start_app_chain() -> Result<(), RunError> {
    let app_chains_list = get_apps_list()?;
    let app = get_option("Select the app chain:", app_chains_list)?;
    let app_chain: &str = &app;

    let (config, _) = match regenerate_app_config(app_chain) {
        Ok((config, valid)) => (config, valid),
        Err(err) => {
            log::error!("Failed to fetch the required app chain: {}", err);
            return Err(RunError::FailedToRegenerateConfig(app_chain.to_string()));
        }
    };

    madara::clone_madara_and_build_repo()?;

    let da_factory = DAFactory::new_da(&config.da_layer);
    da_factory.confirm_minimum_balance(&config)?;
    da_factory.setup(&config).await?;

    madara::setup_and_run_madara(config)?;

    Ok(())
}
