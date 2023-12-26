use serde::Deserialize;
use thiserror::Error;

pub const GITHUB_API_BASE_URL: &str = "https://api.github.com";

#[derive(Debug, Deserialize)]
struct Commit {
    sha: String,
}

#[derive(Debug, Error)]
pub enum GithubError {
    #[error("Failed to get commits from Github")]
    FailedToGetCommits(reqwest::Error),
    #[error("No commits found")]
    NoCommitsFound,
}

pub fn get_latest_commit_hash(org: &str, repo: &str) -> Result<String, GithubError> {
    let github_api_url = format!("{}/repos/{}/{}/commits", GITHUB_API_BASE_URL, org, repo);

    return match reqwest::blocking::get(&github_api_url) {
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
