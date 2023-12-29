use std::path::PathBuf;
use git2::Repository;
use std::process::{Command, Stdio};
use git2::ErrorClass;
use super::utils::{get_karnot_home, git_clone};

pub fn clone_madara_and_build_repo() -> Result<(), std::io::Error> {
    let repo_url = "https://github.com/keep-starknet-strange/madara";
    let madara_path = get_karnot_home().unwrap().join("madara");

    git_clone(repo_url, &madara_path);

    // Change directory to the cloned repository
    std::env::set_current_dir(madara_path);

    // Build the Rust program
    let build_result = Command::new("cargo")
        .arg("build")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output();

    if let Ok(output) = build_result {
        if output.status.success() {
            println!("Build successful!");
        } else {
            eprintln!("Build failed: {:?}", output);
        }
    } else {
        eprintln!("Failed to execute cargo build");
    }

    Ok(())
}
