use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

pub fn execute_cmd(program: &str, args: &[&str], dir: &PathBuf) -> Result<Output, Error> {
    let output = execute_cmd_stdout(program, args, dir, Stdio::inherit())?;
    Ok(output)
}

pub fn execute_cmd_stdout(program: &str, args: &[&str], dir: &PathBuf, out: Stdio) -> Result<Output, Error> {
    let result = Command::new(program).current_dir(dir).args(args).stdout(out).stderr(Stdio::inherit()).output();

    match result {
        Ok(output) => {
            if output.status.success() {
                log::info!("Successfully executed {}", program);
                Ok(output)
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
