use crate as karnot;
use dirs;

pub fn run(app_chain: &Option<String>) {
    // let app_chain = app_chain.unwrap();

    karnot::app::madara::clone_madara_and_build_repo();
    generate_config_file();
    generate_service_file();
}

use std::fs;
use std::io;
use std::process::Command;
use whoami;
use crate::app::utils::{get_app_chains_home, get_templates_dir};

fn generate_file(template_file_name: &str, replacement_map: &[(&str, &str)], output_file_path: &str) -> Result<(), std::io::Error> {
    let template_dir = get_templates_dir()?;
    let template_path = template_dir.join(template_file_name).to_string_lossy().to_string();
    let template_content = fs::read_to_string(template_path).unwrap();

    let mut modified_content = template_content.clone();
    for (placeholder, value) in replacement_map {
        modified_content = modified_content.replace(placeholder, value);
    }

    // Write the modified content to a new .sh file
    fs::write(&output_file_path, modified_content)?;

    println!("Modified script saved at: {}", output_file_path);
    Ok(())
}

fn generate_config_file() -> Result<(), std::io::Error> {
    let template_file_name = "karnot.sh";
    let replacement_map = [
        ("{{BASE_PATH}}", "test"),
    ];
    let output_file_path = "/Users/ashukla/.karnot/";

    generate_file(template_file_name, &replacement_map, output_file_path)
}

fn generate_service_file() -> Result<(), std::io::Error> {
    let template_file_name = "karnot.service";
    let replacement_map = [
        ("{{DESCRIPTION}}", "test"),
        ("{{WORKING_DIRECTORY}}", "/Users/ashukla/.karnot/"),
        ("{{EXEC_START}}", "stark"),
    ];
    let output_file_path = "/Users/ashukla/.karnot/xyz.service";

    generate_file(template_file_name, &replacement_map, output_file_path)
}


fn symlink_to_systemd() -> io::Result<()> {
    let symlink = Command::new("ln")
        .arg("-s")
        .arg("/path/to/custom/location/my_rust_service.service")
        .arg("/etc/systemd/system/my_rust_service.service")
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
