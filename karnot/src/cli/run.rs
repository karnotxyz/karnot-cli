use crate as karnot;
use dirs;

pub fn run() {
    let app = select_app_chain().unwrap();
    let app_chain: &str = &app;

    karnot::app::madara::clone_madara_and_build_repo();
    setup_madara(&app_chain);
}

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use whoami;
use crate::app::config::AppChainConfig;
use crate::app::utils::{get_app_home, get_karnot_home, get_templates_dir};
use crate::cli::prompt::select_app_chain;


fn setup_madara(app_chain: &str) {
    let madara = get_karnot_home().unwrap().join("madara");
    println!("{:?}", madara);

    let mut command = Command::new("cargo");
    let output = command
        .current_dir(&madara)
        .arg("run")
        .arg("--release")
        .arg("setup")
        .arg("--chain=dev")
        .arg("--from-remote")
        .arg("--base-path=/Users/ashukla/.karnot/app-chains/karnot/data")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output();

    run_madara(&madara);
}

fn run_madara(madara: &PathBuf) {
    let madara_binary = madara.join("target/release/madara");
    // madara  --base-path=/Users/ashukla/epf/stark --rpc-cors=all --chain=dev --force-authoring   --rpc-external --rpc-methods=unsafe
    let output = Command::new(madara_binary)
        .arg("--base-path=/Users/ashukla/.karnot/app-chains/karnot/data")
        .arg("--rpc-cors=all")
        .arg("--chain=dev")
        .arg("--force-authoring")
        .arg("--rpc-external")
        .arg("--rpc-methods=unsafe")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output();

    println!("{:?}", output);
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