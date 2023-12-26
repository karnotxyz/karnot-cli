use std::{fs};

use crate as karnot;

use karnot::app::config::AppChainConfig;
use std::path::{Path};
use crate::app::utils::{get_app_chains_home};

pub fn list() {
    let apps = get_apps_list();
    println!("{:?}", apps);
}

/// Assumes that all the app configs are saved at "~/.karnot/app-chains/{app}/{app}-config.toml"
/// But return app names after validating the {app}-config.toml
fn get_apps_list() -> Vec<String> {
    let mut config_paths = Vec::new();
    let app_configs = get_app_chains_home().unwrap();

    if let Ok(entries) = fs::read_dir(&app_configs) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap_or_default();
                let file_path = entry.path().join(format!("{}-config.toml",file_name));
                if check_toml(&file_path) {
                    if let Some(path_str) = file_path.to_str() {
                        config_paths.push(file_name);
                    }
                }
            } else {
                eprintln!("Error reading directory: {:?}", app_configs)
            }
        }
    }

    config_paths
}

fn check_toml(file_path: &Path) -> bool {
    let toml_content = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return false;
        }
    };

    let deserialized_result: Result<AppChainConfig, toml::de::Error> =
        toml::from_str(&toml_content);

    match deserialized_result {
        Ok(_) => true,
        Err(_) => false,
    }
}
