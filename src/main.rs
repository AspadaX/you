mod information;
mod arguments;
mod llm;
mod agent;
mod styles;

use anyhow::{Error, Result};
use cchain::{commons::utility::input_message, display_control::{display_message, Level}};
use clap::{Parser, crate_version, crate_authors, crate_description, crate_name};
use agent::{CommandJSON, Executable, SemiAutonomousCommandLineAgent, Step};
use arguments::{Arguments, Commands};
use indicatif::ProgressBar;
use llm::Context;
use styles::start_spinner;

fn main() -> Result<(), Error> {
    let arguments = Arguments::parse();
    
    match arguments.commands {
        Commands::Run(subcommand) => {
            let mut agent = SemiAutonomousCommandLineAgent::new()?;
            let mut is_first_round: bool = true;
            
            let mut user_prompt = String::new();
            loop {
                // Initialize an empty vector to store command lines
                let mut command_json: CommandJSON;
                
                // Use the user query provided in the `run` argument for the first round
                let spinner: ProgressBar = start_spinner("LLM is thinking...".to_string());
                if is_first_round {
                    is_first_round = false;
                    command_json = agent.next_step(
                        &subcommand.command_in_natural_language
                    )?;
                } else {
                    command_json = agent.next_step(&user_prompt)?;
                }
                
                // Clear the spinner
                spinner.finish_and_clear();
                
                // For prompting the LLM and the user
                let command_lines_text: String = "    > ".to_string() + command_json.command.to_string().as_str() + "\n";
                let command_lines_explanation: String = "        * ".to_string() + command_json.explanation.to_string().as_str() + "\n";
                
                // Register the user's input
                user_prompt = input_message(
                    &format!("Execute the following command? (y for yes, or type to hint LLM)\n{}{}", command_lines_text, command_lines_explanation)
                )?;
                // we add the command lines to the agent's memory
                agent.add(async_openai::types::Role::Assistant, format!("{}{}", command_lines_text, command_lines_explanation))?;
                
                if user_prompt.trim() == "y" {
                    match agent.execute(&mut command_json) {
                        Ok(_) => {
                            display_message(Level::Logging, "Commands had been executed successfully.");
                            break;
                        },
                        Err(error) => {
                            display_message(Level::Error, &error.to_string());
                        }
                    };
                }
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
