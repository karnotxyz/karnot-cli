use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use git2::Repository;
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::utils::errors::GithubError;
pub const GITHUB_API_BASE_URL: &str = "https://api.github.com";

#[derive(Debug, Deserialize)]
struct Commit {
    sha: String,
}

pub fn get_latest_commit_hash(org: &str, repo: &str) -> Result<String, GithubError> {
    let github_api_url = format!("{}/repos/{}/{}/commits", GITHUB_API_BASE_URL, org, repo);

    let client = Client::new();
    let response = client.get(github_api_url).header("User-Agent", "reqwest").send();

    return match response {
        Ok(response) => match response.json::<Vec<Commit>>() {
            Ok(commits) => match commits.first() {
                Some(latest_commit) => Ok(latest_commit.sha.clone()),
                None => Err(GithubError::NoCommitsFound),
            },
            Err(err) => Err(GithubError::FailedToGetCommits(err)),
        },
        Err(err) => Err(GithubError::FailedToGetCommits(err)),
    };
}

pub fn git_clone(url: &str, path: &PathBuf) -> Result<(), GithubError> {
    if let Ok(repo) = Repository::open(path) {
        // Check if the repository is valid
        if repo.is_empty() == Ok(false) {
            let remote = repo.find_remote("origin")?;
            if let Some(remote_url) = remote.url() {
                if remote_url == url {
                    return Ok(());
                }
            }
        }
    }

    if path.exists() {
        log::info!("Detected an issue with the Madara repository");
        log::info!("Initiating removal and re-cloning process");
        fs::remove_dir_all(path)?;
    }

    let output = Command::new("git")
        .arg("clone")
        .arg("--progress")
        .arg(url)
        .arg(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    let status = output.status;

    if status.success() {
        log::info!("Clone successful!");
        Ok(())
    } else {
        log::error!("Clone failed");
        Err(GithubError::FailedToCloneRepo)
    }
}
