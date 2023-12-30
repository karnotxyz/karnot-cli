use dirs;
use std::fs;
use whoami;
use crate::app::config::AppChainConfig;
use crate::utils::paths::{get_app_home, get_karnot_home};
use crate::cli::list::get_apps_list;
use crate::cli::prompt::get_option;
use crate::utils::cmd::execute_cmd;
use crate::utils::madara;

pub fn run() {
    let ac = get_apps_list();
    let app = get_option("Select the app chain:", ac).unwrap();
    let app_chain: &str = &app;

    madara::clone_madara_and_build_repo().unwrap();
    setup_madara(&app_chain);
}

fn setup_madara(app_chain: &str) {
    let madara = get_karnot_home().unwrap().join("madara");
    let config = app_config(&app_chain).unwrap();
    let base_path = format!("--base-path={}", config.base_path);
    execute_cmd(
        "cargo",
        &["run", "--release", "setup", "--chain=dev", "--from-remote", &base_path],
        &madara)
        .unwrap();

    run_madara(&app_chain);
}

fn run_madara(app_chain: &str) {
    let madara = get_karnot_home().unwrap().join("madara");
    let config = app_config(&app_chain).unwrap();
    let base_path = format!("--base-path={}", config.base_path);
    execute_cmd(
        "./target/release/madara",
        &["--rpc-cors=all", "--chain=dev", "--force-authoring", "--rpc-external", "--rpc-methods=unsafe", &base_path],
        &madara
    ).unwrap();
}

fn app_config(app: &str) -> Result<AppChainConfig, toml::de::Error> {
    let app_home = get_app_home(&app).unwrap();
    let app_config = app_home.join(format!("{}-config.toml",app));
    let toml_content = match fs::read_to_string(&app_config) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            let s = String::from("");
            s
        }
    };

    toml::from_str(&toml_content)
}