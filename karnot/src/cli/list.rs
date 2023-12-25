use std::fs;

use crate as karnot;

use karnot::app::config::AppChainConfig;

pub fn list() {
    get_app_configs();
}

/// Assumes that all the app configs are saved at "~/.karnot"
fn get_app_configs() {
    let home = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Unable to get the home directory.");
            return;
        }
    };

    let karnot_config = home.join(".karnot");

    if let Ok(entries) = fs::read_dir(&karnot_config) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap_or_default();
                let file_path = entry.path().into_os_string().into_string().unwrap();
                if file_name.ends_with(".toml") && check_toml(&file_path) {
                    println!("Config found: {}", file_name);
                }
            } else {
                eprintln!("Error reading directory: {:?}", karnot_config)
            }
        }
    }
}

pub fn check_toml(file: &str) -> bool {
    let toml_content = match fs::read_to_string(&file) {
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
