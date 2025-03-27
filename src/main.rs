mod information;
mod arguments;
mod llm;
mod agent;
mod styles;

use std::time::Duration;

use anyhow::{Error, Result};
use cchain::{commons::utility::input_message, display_control::{display_message, Level}};
use clap::{Parser, crate_version, crate_authors, crate_description, crate_name};
use agent::{CommandJSON, Executable, SemiAutonomousCommandLineAgent, Step};
use arguments::{Arguments, Commands};
use llm::Context;
use styles::start_spinner;

fn main() -> Result<(), Error> {
    let arguments = Arguments::parse();
    
    match arguments.commands {
        Commands::Run(subcommand) => {
            let mut agent = SemiAutonomousCommandLineAgent::new()?;
            let mut is_first_round: bool = true;
            
            let mut user_input_in_previous_round = String::new();
            let mut error_message_in_previous_round = String::new();
            loop {
                // Initialize an empty vector to store command lines
                let mut command_json: CommandJSON;
                
                // Use the user query provided in the `run` argument for the first round
                let spinner = start_spinner("LLM is thinking...".to_string());
                if is_first_round {
                    is_first_round = false;
                    
                    command_json = agent.next_step(
                        &subcommand.command_in_natural_language
                    )?;
                    
                    spinner.finish_and_clear();
                } else {
                    if error_message_in_previous_round != "" {
                        command_json = agent.next_step(&error_message_in_previous_round)?;
                        error_message_in_previous_round.clear();
                        spinner.finish_and_clear();
                    } else {
                        command_json = agent.next_step(&user_input_in_previous_round)?;
                        user_input_in_previous_round.clear();
                        spinner.finish_and_clear();
                    }
                }
                
                let command_lines_text: String = "    > ".to_string() + command_json.command.to_string().as_str() + "\n";
                let command_lines_explanation: String = "        * ".to_string() + command_json.explanation.to_string().as_str() + "\n";
                let user_input: String = input_message(
                    &format!("Execute the following command? (y for yes, or type to hint LLM)\n{}{}", command_lines_text, command_lines_explanation)
                )?;
                
                if user_input.trim() == "y" {
                    match agent.execute(&mut command_json) {
                        Ok(_) => {
                            display_message(Level::Logging, "Commands had been executed successfully.");
                            break;
                        },
                        Err(error) => {
                            display_message(Level::Error, &error.to_string());
                            let prompt: String = format!(
                                "Failed to execute the following command(s):\n{}\n\nWith following error: {}", 
                                command_lines_text, 
                                error
                            );
                            // Record the error message to be sent in the next round
                            error_message_in_previous_round = prompt;
                            continue;
                        }
                    };
                }
                
                // Add the assistant generated command to the agent's memory
                agent.add(async_openai::types::Role::Assistant, command_lines_text)?;
                
                // Store the user input in the previous round
                user_input_in_previous_round = user_input;
            }
        },
        Commands::Version(_) => {
            display_message(Level::Logging, &format!("{}", crate_name!()));
            display_message(Level::Logging, &format!("version.{}", crate_version!()));
            display_message(Level::Logging, &format!("Authors: {}", crate_authors!()));
            display_message(Level::Logging, &format!("Description: {}", crate_description!()));
        }
    }
    
    Ok(())
}
