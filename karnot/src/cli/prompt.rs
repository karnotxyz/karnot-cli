use inquire::error::InquireResult;
use inquire::{Confirm, CustomType, InquireError, Select, Text};
use strum::IntoEnumIterator;

use crate as karnot;

use karnot::app::config::{DALayer, RollupMode};
use crate::cli::list::get_apps_list;

pub fn ask_for_app_chain_name() -> Result<String, InquireError> {
    Text::new("Enter you app chain name:")
        .with_default("karnot")
        .prompt()
}

pub fn select_app_chain() -> Result<String, InquireError> {
    let app_chains = get_apps_list();
    Select::new("Select app you want to run:", app_chains).prompt()
}

pub fn ask_for_base_path() -> Result<String, InquireError> {
    Text::new("Enter base path for data directory of your app chain:")
        .with_default("karnot")
        .prompt()
}

pub fn ask_for_chain_id() -> Result<String, InquireError> {
    Text::new("Enter chain id for your app chain:")
        .with_default("karnot")
        .prompt()
}

pub fn ask_for_mode() -> Result<RollupMode, InquireError> {
    let modes = RollupMode::iter().collect::<Vec<_>>();
    Select::new("Select mode for your app chain:", modes).prompt()
}

pub fn ask_for_da_layer() -> Result<DALayer, InquireError> {
    let da = DALayer::iter().collect::<Vec<_>>();
    Select::new("Select mode for your app chain:", da).prompt()
}

pub fn ask_for_block_time() -> InquireResult<u64> {
    CustomType::new("Enter block time of chain:")
        .with_default(6000)
        .with_help_message("Time in ms (e.g, 1000, 2000)")
        .prompt()
}

pub fn ask_for_disable_fees() -> Result<bool, InquireError> {
    Confirm::new("Do you want to disable fees for your app chain:")
        .with_default(true)
        .prompt()
}

pub fn ask_for_fee_token() -> Result<String, InquireError> {
    Text::new("Enter fee token:").with_default("KAR").prompt()
}
