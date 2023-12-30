use inquire::InquireError;
use crate::cli::list::get_apps_list;
use crate::cli::prompt::get_option;
use crate::utils::madara;
use crate::utils::errors::MadaraError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunError {
    #[error("Failed to get input: {0}")]
    FailedToGetInout(#[from] InquireError),
    #[error("Failed to start madara: {0}")]
    FailedToStartMadara(#[from] MadaraError),
    #[error("Failed to get app chains: {0}")]
    FailedToGetAppChains(#[from] std::io::Error),
}
pub fn run() {
    match start_app_chain() {
        Ok(_) => {
            println!("Madara setup successful");
        },
        Err(err) => {
            log::error!("Failed to setup Madara: {:?}", err);
        }
    }
}

fn start_app_chain() -> Result<(), RunError> {
    let app_chains_list = get_apps_list()?;
    let app = get_option("Select the app chain:", app_chains_list)?;
    let app_chain: &str = &app;

    madara::clone_madara_and_build_repo()?;
    madara::setup_and_run_madara(&app_chain)?;

    Ok(())
}