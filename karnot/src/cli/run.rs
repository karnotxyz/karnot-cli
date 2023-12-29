use crate as karnot;
use dirs;

pub fn run() {
    let app = select_app_chain().unwrap();
    let app_chain: &str = &app;

    karnot::app::madara::clone_madara_and_build_repo();
    generate_sh_file(&app_chain);
    execute_shell_script(&app_chain);
    // // generate_service_file(app_chain);
}

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use whoami;
use crate::app::config::AppChainConfig;
use crate::app::utils::{get_app_home, get_templates_dir};
use crate::cli::prompt::select_app_chain;

fn generate_file(template_file_name: &str, replacement_map: &[(&str, &str)], output_file_path: &PathBuf) -> Result<(), std::io::Error> {
    let template_dir = get_templates_dir()?;
    let template_path = template_dir.join(template_file_name).to_string_lossy().to_string();
    let template_content = fs::read_to_string(template_path).unwrap();

    let mut modified_content = template_content.clone();
    for (placeholder, value) in replacement_map {
        modified_content = modified_content.replace(placeholder, value);
    }

    println!("{:?}", output_file_path);
    fs::write(&output_file_path, modified_content)?;

    println!("Modified script saved at: {:?}", output_file_path);
    Ok(())
}

fn generate_sh_file(app_chain: &str) -> Result<(), std::io::Error> {
    let template_file_name = "karnot.sh";
    let app_home = get_app_home(app_chain)?;
    let config = app_config(&app_chain).unwrap();
    let base_path: &str = &config.base_path;
    let replacement_map = [
        ("{{BASE_PATH}}", base_path),
    ];

    let sh_file = &format!("{}-karnot.sh", app_chain);
    let output_file_path = app_home.join(&sh_file);

    generate_file(template_file_name, &replacement_map, &output_file_path)
}


fn generate_service_file(app_chain: &str) -> Result<(), std::io::Error> {
    let template_file_name = "karnot.service";
    let app_home = get_app_home(app_chain)?;
    let working_dir = app_home.to_str().unwrap_or_default();
    let service_file = format!("{}-karnot.service", app_chain);
    let service_file_str: &str = &service_file;

    let replacement_map = [
        ("{{DESCRIPTION}}", app_chain),
        ("{{WORKING_DIRECTORY}}", working_dir),
        ("{{EXEC_START}}", service_file_str),
    ];

    let output_file_path = app_home.join(&service_file);
    generate_file(template_file_name, &replacement_map, &output_file_path);
    symlink_to_systemd(&output_file_path, &service_file)
}

fn execute_shell_script(app_chain: &str) {
    let app = get_app_home(&app_chain).unwrap();
    let script_path = app.join(format!("{}-karnot.sh", &app_chain));
    let output = Command::new("sh")
        .arg(script_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute the script");

    // Check if the command was successful
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Script executed successfully. Output: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Script failed to execute. Error: {}", stderr);
    }
}

fn symlink_to_systemd(file_path: &PathBuf, file_name: &str) -> Result<(), std::io::Error> {
    let sys = format!("/etc/systemd/system/{}", file_name);
    let symlink = Command::new("ln")
        .arg("-s")
        .arg(file_path)
        .arg(sys)
        .output()?;

    if symlink.status.success() {
        println!("Service file symlinked successfully.");
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Symlink creation failed."))
    }
}

fn enable_and_start_service() -> io::Result<()> {
    let reload = Command::new("systemctl").arg("daemon-reload").output()?;
    println!("{}", String::from_utf8_lossy(&reload.stdout));

    let enable = Command::new("systemctl").arg("enable").arg("my_rust_service.service").output()?;
    println!("{}", String::from_utf8_lossy(&enable.stdout));

    let start = Command::new("systemctl").arg("start").arg("my_rust_service.service").output()?;
    println!("{}", String::from_utf8_lossy(&start.stdout));

    Ok(())
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