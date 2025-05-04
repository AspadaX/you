use anyhow::{Error, Result};
use cchain::{
    commons::utility::input_message,
    display_control::{display_message, Level},
};
use indicatif::ProgressBar;

use crate::{
    agents::{
        command_json::{ActionTypeExecute, LLMActionType}, command_line_explain_agent::CommandLineExplainAgent,
        semi_autonomous_command_line_agent::SemiAutonomousCommandLineAgent, traits::{AgentExecution, Step},
    },
    llm::Context,
    styles::start_spinner,
};

/// Prepares and displays a command prompt to the user, asking for confirmation or additional input
/// 
/// # Arguments
/// * `command_json` - The command JSON containing the command and its explanation
/// 
/// # Returns
/// * `Result<String>` - The user's input response
pub fn prompt_user_for_command_execution(command_json: &LLMActionType) -> Result<String> {
    // Get the display prompt from the LLMActionType
    let prompt: String = command_json.fetch_display_prompt();

    // Prompt the user for input
    input_message(&prompt)
}

fn process_command_interaction(
    agent: &mut (impl Context + Step<LLMActionType>),
    user_prompt: &mut String
) -> Result<LLMActionType, Error> {
    // Use the user query provided in the `run` argument for the first round
    let spinner: ProgressBar = start_spinner("LLM is thinking...".to_string());
    let command_json: LLMActionType = agent.next_step(&user_prompt)?;
    // Clear the spinner
    spinner.finish_and_clear();

    // Update the user prompt based on command type
    *user_prompt = prompt_user_for_command_execution(&command_json)?;

    // we add the `LLMActionType` to the agent's memory
    agent.add(
        async_openai::types::Role::Assistant,
        format!("{:#?}", command_json),
    )?;

    Ok(command_json)
}

pub fn process_run_with_one_single_instruction(
    command_in_natural_language: &str,
) -> Result<(), Error> {
    let mut agent: SemiAutonomousCommandLineAgent = SemiAutonomousCommandLineAgent::new()?;
    let mut user_prompt: String = String::from(command_in_natural_language);

    loop {
        // Process the command interaction
        let mut command_json: LLMActionType = process_command_interaction(
            &mut agent, 
            &mut user_prompt
        )?;

        if user_prompt.trim() == "y" {
            match command_json.execute() {
                Ok(_) => {
                    display_message(Level::Logging, "Commands had been executed successfully.");

                    // Prompt the user for saving the command
                    let save_shell_input: String = input_message(
                        "Would you like to save the command to a chain? (n for no, type anything to name the chain)",
                    )?;
                    if save_shell_input.trim() == "n" {
                        break;
                    }

                    // Extract command if it's an Execute action type
                    if let LLMActionType::Execute(ref mut execute_action) = command_json {
                        save_to_shell(save_shell_input.trim(), execute_action)?;
                    }
                    break;
                }
                Err(error) => {
                    let error_string: String = error.to_string();
                    display_message(Level::Error, &error_string);
                    user_prompt.push_str(&error_string);
                }
            };
        }
    }

    Ok(())
}

pub fn process_interactive_mode() -> Result<(), Error> {
    let mut agent: SemiAutonomousCommandLineAgent = SemiAutonomousCommandLineAgent::new()?;
    let mut command_store: LLMActionType;
    let mut user_query: String = input_message("Yes, boss. What can I do for you:")?;

    loop {
        let mut command_json: LLMActionType = process_command_interaction(
            &mut agent, 
            &mut user_query
        )?;

        if user_query.trim() == "y" {
            match command_json.execute() {
                Ok(result) => {
                    // Store the command
                    command_store = command_json;
                    // Store the output to the user_query
                    user_query.clear();
                    user_query.push_str(&format!(
                        "Here is the previous output of the command/script:\n{}\n\n",
                        result
                    ));

                    display_message(Level::Logging, "Commands had been executed successfully.");

                    let user_input: String = input_message(
                        "Boss, what else can I do for you (type to instruct, e to exit, or enter w to save the commands so far):",
                    )?;

                    if user_input.trim() == "e" {
                        break;
                    }

                    if user_input.trim() == "w" {
                        let name: String = input_message("Name of the chain:")?;

                        // Extract command if it's an Execute action type
                        if let LLMActionType::Execute(ref mut execute_action) = command_store {
                            save_to_shell(name.trim(), execute_action)?;
                        }

                        let user_feedback: String =
                            input_message("Continue? (y for yes, e for exit):")?;

                        if user_feedback.trim() == "e" {
                            break;
                        }

                        if user_feedback.trim() == "y" {
                            user_query.push_str(&input_message(
                                "Boss, what else can I do for you (type to instruct):",
                            )?);
                        }
                    }

                    user_query.push_str(&user_input);
                }
                Err(error) => {
                    display_message(Level::Error, &error.to_string());
                }
            };
        }

        if user_query.trim() == "e" {
            display_message(Level::Logging, "See you boss.");
            break;
        }
    }

    Ok(())
}

pub fn process_explanation_with_one_single_instruction(
    command: &str,
) -> Result<(), Error> {
    let mut agent = CommandLineExplainAgent::new()?;

    // Use the user query provided in the `run` argument for the first round
    let spinner: ProgressBar = start_spinner("LLM is thinking...".to_string());
    let command_line_explained = agent.next_step(&command)?;

    // Clear the spinner
    spinner.finish_and_clear();

    // For prompting the LLM and the user
    let command_lines_explanation: String = command_line_explained.to_string() + "\n";
    
    display_message(Level::Logging, &command_lines_explanation);
    
    Ok(())
}

fn save_to_shell(shell_name: &str, execute_action: &mut ActionTypeExecute) -> Result<(), Error> {
    let mut file_content: String = String::from("#!/usr/bin/env sh\n");
    file_content.push_str(execute_action.get_commands());

    let filepath: &str = &format!("./{}.sh", shell_name);
    std::fs::write(filepath, file_content)?;
    display_message(
        Level::Logging,
        &format!("Shell had been saved to {}.", filepath),
    );

    Ok(())
}
