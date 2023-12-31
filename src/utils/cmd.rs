use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn execute_cmd(program: &str, args: &[&str], dir: &PathBuf) -> Result<(), Error> {
    let result =
        Command::new(program).current_dir(dir).args(args).stdout(Stdio::inherit()).stderr(Stdio::inherit()).output();

    match result {
        Ok(output) => {
            if output.status.success() {
                log::info!("Successfully executed {}", program);
                Ok(())
            } else {
                Err(Error::new(ErrorKind::Other, "Unable to execute command"))
            }
        }
        Err(err) => {
            log::error!("Error executing {}", program);
            Err(err)
        }
    }
}
