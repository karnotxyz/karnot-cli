use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub fn get_madara_home() -> Result<PathBuf, Error> {
    if let Some(home_dir) = dirs::home_dir() {
        let madara_home = home_dir.join(".madara");

        // Creates the `madara_home` directory if not present
        fs::create_dir_all(&madara_home)?;

        return Ok(madara_home);
    }

    Err(Error::new(ErrorKind::NotFound, "Failed to get the home directory"))
}

pub fn get_app_chains_home() -> Result<PathBuf, Error> {
    let madara_home = get_madara_home()?;
    let app_chains = madara_home.join("app-chains");

    // Creates the `app-chain` directory if not present
    fs::create_dir_all(&app_chains)?;

    Ok(app_chains)
}

pub fn get_app_home(app: &str) -> Result<PathBuf, Error> {
    let app_chains = get_app_chains_home()?;
    let app_home = app_chains.join(app);

    // Creates the `app_home` directory if not present
    fs::create_dir_all(&app_home)?;

    Ok(app_home)
}
