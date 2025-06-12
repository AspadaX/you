use std::io::Read;

use anyhow::{Error, Result, anyhow};
use cchain::display_control::{display_command_line, display_message};
use serde::{Deserialize, Serialize};

use super::traits::AgentExecution;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CLIToInstall {
    pub cli_name: String,
    pub suggested_installation_command: String,
    pub additional_notices: Option<String>,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum LLMActionType {
    Execute(ActionTypeExecute),
    RequestInformation(ActionTypeRequestInformation),
    RequestCLIsToInstall(ActionTypeRequestCLIsToInstall),
}

impl LLMActionType {
    /// The enum `LLMActionType` represents different types of actions that can be performed by an LLM agent.
    ///
    /// # Variants
    ///
    /// * `Execute` - Executes a shell command with an explanation.
    /// * `RequestInformation` - Requests additional information from the user.
    /// * `InstallDependencies` - Handles installation of CLI dependencies.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::agents::command_json::{LLMActionType, ActionTypeExecute};
    ///
    /// // Create an execute action
    /// let action = LLMActionType::Execute(ActionTypeExecute {
    ///     command: "ls -la".to_string(),
    ///     explanation: "List all files in the current directory with details".to_string(),
    /// });
    ///
    /// // You can use pattern matching to handle different action types
    /// match action {
    ///     LLMActionType::Execute(exec) => println!("Command to execute: {}", exec.get_commands()),
    ///     LLMActionType::RequestInformation(_) => println!("Need more information from user"),
    ///     LLMActionType::InstallDependencies(_) => println!("Need to install dependencies"),
    /// }
    /// ```
    pub fn get_llm_action_type_prompt_template() -> String {
        // We use the default impl to fetch their corresponding prompts
        let execution_template: ActionTypeExecute = ActionTypeExecute::default();
        let request_information_template: ActionTypeRequestInformation =
            ActionTypeRequestInformation::default();
        let request_clis_template: ActionTypeRequestCLIsToInstall =
            ActionTypeRequestCLIsToInstall::default();

        // Convert them into json strings
        let execution_json: String = serde_json::to_string(&execution_template).unwrap_or_default();
        let request_information_json: String =
            serde_json::to_string(&request_information_template).unwrap_or_default();
        let request_clis_json: String =
            serde_json::to_string(&request_clis_template).unwrap_or_default();

        let mut prompt: String = String::new();
        prompt.push_str(&format!(
            "To execute a command, you may output: {}",
            execution_json
        ));
        prompt.push_str(&format!(
            "\n\nTo request additional information from the user, you may output: {}",
            request_information_json
        ));
        prompt.push_str(&format!(
            "\n\nIf the command is not found, you may output: {}",
            request_clis_json
        ));

        prompt
    }

    /// Returns a formatted prompt string based on the action type for display to the user.
    ///
    /// This method generates appropriate text prompts for different LLM action types:
    /// - For `Execute` actions, it creates a command execution confirmation prompt
    /// - For `RequestInformation` actions, it returns the request for additional information
    /// - For `InstallDependencies` actions, it lists tools that need installation
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted prompt to display to the user
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::agents::command_json::{LLMActionType, ActionTypeExecute};
    ///
    /// // Create an execute action
    /// let action = LLMActionType::Execute(ActionTypeExecute {
    ///     command: "ls -la".to_string(),
    ///     explanation: "List all files with details".to_string(),
    /// });
    ///
    /// // Get the display prompt
    /// let prompt = action.fetch_display_prompt();
    /// assert!(prompt.contains("ls -la"));
    /// assert!(prompt.contains("List all files with details"));
    /// ```
    pub fn fetch_display_prompt(&self) -> String {
        match self {
            Self::Execute(execute_action) => {
                format!(
                    "Your input: (y for executing the command, or type to hint LLM)\n    > {}\n        * {}\n",
                    execute_action.command, execute_action.explanation
                )
            }
            Self::RequestInformation(request_info) => {
                request_info.request_additional_information.clone()
            }
            Self::RequestCLIsToInstall(request_clis) => {
                let mut prompt: String = String::new();
                prompt.push_str("The following CLI tools need to be installed:\n\n");

                for cli in &request_clis.request_clis_to_install {
                    prompt.push_str(&format!("Tool: {}\n", cli.cli_name));
                    prompt.push_str(&format!(
                        "Installation Command: {}\n",
                        cli.suggested_installation_command
                    ));

                    if let Some(notice) = &cli.additional_notices {
                        prompt.push_str(&format!("Note: {}\n", notice));
                    }

                    prompt.push_str("\n");
                }

                prompt.push_str("Please install these tools and then continue.");
                prompt
            }
        }
    }
}

impl AgentExecution for LLMActionType {
    fn execute(&mut self) -> Result<String, Error> {
        match self {
            Self::Execute(execute_action) => {
                // Execute the command using the Execute action type's implementation
                execute_action.execute()
            }
            Self::RequestInformation(_) => Err(anyhow!("Cannot execute a request for information")),
            Self::RequestCLIsToInstall(_) => {
                Err(anyhow!("Cannot execute a request to install CLI tools"))
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActionTypeExecute {
    command: String,
    explanation: String,
}

impl AgentExecution for ActionTypeExecute {
    fn execute(&mut self) -> Result<String, Error> {
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
        let mut child: std::process::Child = command
            .spawn()
            .map_err(|e| anyhow!("Failed to execute command: {}", e))?;

        // Take the stdout and stderr handles
        let stdout: std::process::ChildStdout = child.stdout.take().unwrap();
        let stderr: std::process::ChildStderr = child.stderr.take().unwrap();

        let (tx, rx) = std::sync::mpsc::channel();

        // Spawn a thread to read stdout
        let tx_clone: std::sync::mpsc::Sender<String> = tx.clone();
        std::thread::spawn(move || {
            let mut reader: std::io::BufReader<std::process::ChildStdout> =
                std::io::BufReader::new(stdout);
            let mut buffer: [u8; 1024] = [0; 1024];
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
            let mut reader: std::io::BufReader<std::process::ChildStderr> =
                std::io::BufReader::new(stderr);
            let mut buffer: [u8; 1024] = [0; 1024];
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

        let mut collected_output: String = String::new();
        let terminal: console::Term = console::Term::stdout();
        for received in rx {
            display_command_line(&terminal, &received);
            collected_output.push_str(&received);
        }

        // Wait for process completion
        let status: std::process::ExitStatus = child
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
}

impl ActionTypeExecute {
    pub fn get_commands(&self) -> &str {
        &self.command
    }
}

impl Default for ActionTypeExecute {
    fn default() -> Self {
        Self {
            explanation: "explain the shell script briefly. one line maximum. ".to_string(),
            command: "a shell script, preferrably in one line, to execute.".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActionTypeRequestInformation {
    request_additional_information: String,
}

impl Default for ActionTypeRequestInformation {
    fn default() -> Self {
        Self {
            request_additional_information:
                "Describe what information you want the user to add on. ".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActionTypeRequestCLIsToInstall {
    request_clis_to_install: Vec<CLIToInstall>,
}

impl Default for ActionTypeRequestCLIsToInstall {
    fn default() -> Self {
        Self {
            request_clis_to_install: vec![CLIToInstall::default()],
        }
    }
}
