use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use git2::{Repository};


pub fn get_karnot_home() -> Result<PathBuf, std::io::Error> {
    if let Some(home_dir) = dirs::home_dir() {
        let karnot_home = home_dir.join(".karnot");
        fs::create_dir_all(&karnot_home)?;
        return Ok(karnot_home);
    }

    Err(std::io::Error::new(ErrorKind::NotFound, "Failed to get the home directory"))
}

pub fn get_app_chains_home() -> Result<PathBuf, std::io::Error> {
    let karnot_home = get_karnot_home()?;
    let app_chains = karnot_home.join("app-chains");

    // Creates the `app-chain` directory if not present
    fs::create_dir_all(&app_chains)?;

    Ok(app_chains)
}

pub fn get_app_home(app: &str) -> Result<PathBuf, std::io::Error> {
    let app_chains = get_app_chains_home()?;
    let app_home = app_chains.join(app);

    // Creates the $app_home directory if not present
    fs::create_dir_all(&app_home)?;

    Ok(app_home)
}
