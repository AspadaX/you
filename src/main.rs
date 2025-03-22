mod information;
mod arguments;
mod llm;
mod agent;

use anyhow::{Error, Result};
use cchain::{commons::utility::input_message, core::command::CommandLine, display_control::{display_message, Level}};
use clap::{Parser, crate_version, crate_authors, crate_description, crate_name};
use agent::{Executable, SemiAutonomousCommandLineAgent, Step};
use arguments::{Arguments, Commands};
use llm::Context;

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
                let command_lines: Vec<&CommandLine>;
                
                // Use the user query provided in the `run` argument for the first round
                if is_first_round {
                    is_first_round = false;
                    display_message(Level::Logging, "LLM is thinking...");
                    command_lines = agent.next_step(
                        &subcommand.command_in_natural_language
                    )?;
                } else {
                    if error_message_in_previous_round != "" {
                        display_message(Level::Logging, "Error message received, LLM is thinking...");
                        command_lines = agent.next_step(&error_message_in_previous_round)?;
                        error_message_in_previous_round.clear();
                    } else {
                        display_message(Level::Logging, "LLM is thinking...");
                        command_lines = agent.next_step(&user_input_in_previous_round)?;
                        user_input_in_previous_round.clear();
                    }
                }
                
                let command_lines_text: String = command_lines.iter()
                    .map(|command| "    > ".to_string() + command.to_string().as_str())
                    .collect::<Vec<String>>()
                    .join("\n");
                let user_input: String = input_message(
                    &format!("Execute the following command(s)? (y for yes, or type to hint LLM)\n{}", command_lines_text)
                )?;
                
                if user_input.trim() == "y" {
                    match agent.execute() {
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
