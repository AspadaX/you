use std::io::Read;

use anyhow::{Error, Result, anyhow};
use cchain::display_control::{display_command_line, display_message};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CLIToInstall {
    pub cli_name: String,
    pub suggested_installation_command: String,
    pub additional_notices: Option<String>
}

impl Default for CLIToInstall {
    fn default() -> Self {
        Self { 
            cli_name: "The name of the cli that you want the user to install".to_string(), 
            suggested_installation_command: "Base on the current system platform, suggest a command line for the user to install the tool".to_string(), 
            additional_notices: Some("Leave a notice if any to the user. Leave it null if you don't have a notice".to_string())
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandJSON {
    pub explanation: String,
    pub command: String,
    pub request_additional_information: Option<String>,
    pub request_clis_to_install: Vec<CLIToInstall>
}

impl Default for CommandJSON {
    fn default() -> Self {
        Self {
            explanation: "explain the shell script briefly. one line maximum. "
                .to_string(),
            command: "a shell script, preferrably in one line, to execute."
                .to_string(),
            request_additional_information: Some(
                "Describe what information you want the user to add on. Leave it null if you don't need."
                    .to_string()
            ),
            request_clis_to_install: vec![CLIToInstall::default()]
        }
    }
}

impl CommandJSON {
    pub fn execute(&mut self) -> Result<String, Error> {
        let mut command: std::process::Command = if cfg!(target_os = "windows") {
            let mut cmd = std::process::Command::new("cmd");
            cmd.args(["/C", &self.command]);
            cmd
        } else {
            let mut sh = std::process::Command::new("sh");
            sh.args(["-c", &self.command]);
            sh
        };

        // Set stdout and stderr to piped so that we can capture them
        command.stdout(std::process::Stdio::piped());
        command.stderr(std::process::Stdio::piped());

        let command_in_text: String = format!(r#"{}"#, &self.command);
        let command_string: &console::StyledObject<&String> =
            &console::style(&command_in_text).bold();
        display_message(
            cchain::display_control::Level::Logging,
            &format!("Start executing command: {}", command_string),
        );

        // Spawn the process
        let mut child = command
            .spawn()
            .map_err(|e| anyhow!("Failed to execute command: {}", e))?;

        // Take the stdout and stderr handles
        let stdout: std::process::ChildStdout = child.stdout.take().unwrap();
        let stderr: std::process::ChildStderr = child.stderr.take().unwrap();

        let (tx, rx) = std::sync::mpsc::channel();

        // Spawn a thread to read stdout
        let tx_clone: std::sync::mpsc::Sender<String> = tx.clone();
        std::thread::spawn(move || {
            let mut reader = std::io::BufReader::new(stdout);
            let mut buffer = [0; 1024];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        let text = String::from_utf8_lossy(&buffer[..n]).to_string();
                        tx_clone.send(text).unwrap();
                    }
                    Err(_) => break,
                }
            }
        });

        // Spawn a thread to read stderr
        std::thread::spawn(move || {
            let mut reader = std::io::BufReader::new(stderr);
            let mut buffer = [0; 1024];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        let text = String::from_utf8_lossy(&buffer[..n]).to_string();
                        tx.send(text).unwrap();
                    }
                    Err(_) => break,
                }
            }
        });

        let mut collected_output = String::new();
        let terminal: console::Term = console::Term::stdout();
        for received in rx {
            display_command_line(&terminal, &received);
            collected_output.push_str(&received);
        }

        // Wait for process completion
        let status = child
            .wait()
            .map_err(|e| anyhow!("Failed to wait on child process: {}", e))?;

        if !status.success() {
            return Err(anyhow!("Process exited with non-zero status: {}", status));
        }

        display_message(
            cchain::display_control::Level::Logging,
            &format!("Finished executing command: {}", command_string),
        );

        Ok(collected_output)
    }

    pub fn get_commands(&self) -> &str {
        &self.command
    }
}
