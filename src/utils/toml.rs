use std::{fs};
use crate::app::config::AppChainConfig;
use crate::utils::errors::TomlError;
use crate::utils::paths::get_app_home;

pub fn regenerate_app_config(app: &str) -> Result<(AppChainConfig,bool), TomlError> {
    let app_home = get_app_home(&app)?;
    let app_config = app_home.join(format!("{}-config.toml",app));
    let toml_content = fs::read_to_string(&app_config)?;

    let deserialized_result: Result<AppChainConfig, toml::de::Error> =
        toml::from_str(&toml_content);

    match deserialized_result {
        Ok(app_chain_config) => {
            Ok((app_chain_config,true))
        },
        Err(err) => {
            Err(TomlError::FailedToParseToml(err))
        }
    }
}