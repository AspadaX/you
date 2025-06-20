use std::process::Command;

use anyhow::{Error, Result, anyhow};

/// Execute a shell script with the specified execution context
pub fn execute_shell_script(shell_script: &str) -> Result<(), Error> {
    let current_working_directory: std::path::PathBuf = std::env::current_dir()?;
    if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", shell_script]).current_dir(&current_working_directory);

        match cmd.status() {
            Ok(status) if !status.success() => {
                return Err(anyhow!(
                    "Windows CMD interpreter exited with a non-zero status"
                ));
            }
            Ok(_) => {}
            Err(e) => {
                return Err(anyhow!("Failed to start Windows CMD interpreter: {}", e));
            }
        }

        return Ok(());
    }

    let mut cmd: Command = Command::new("sh");
    cmd.args(["-c", shell_script]).current_dir(&current_working_directory);

    match cmd.status() {
        Ok(status) if !status.success() => {
            return Err(anyhow!("Shell interpreter exited with a non-zero status"));
        }
        Ok(_) => {}
        Err(e) => {
            return Err(anyhow!("Failed to start shell interpreter: {}", e));
        }
    }

    Ok(())
}
