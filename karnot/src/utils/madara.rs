use crate::utils::paths::{get_karnot_home};
use crate::utils::github::git_clone;
use crate::utils::cmd::execute_cmd;

pub fn clone_madara_and_build_repo() -> Result<(), std::io::Error> {
    let repo_url = "https://github.com/keep-starknet-strange/madara";
    let madara_path = get_karnot_home().unwrap().join("madara");

    git_clone(repo_url, &madara_path).unwrap();
    execute_cmd("cargo", &["build", "--release"], &madara_path)
}
