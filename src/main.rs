use anyhow::{Error, Result};
use cchain::{commons::utility::input_message, core::command::CommandLine, display_control::{display_message, Level}};
use clap::{Parser, crate_version, crate_authors, crate_description, crate_name};
use you::{agent::{Executable, SemiAutonomousCommandLineAgent, Step}, arguments::{Arguments, Commands}, llm::Context};

fn main() -> Result<(), Error> {
    let arguments = Arguments::parse();
    
    match arguments.commands {
        Commands::Run(subcommand) => {
            let mut agent = SemiAutonomousCommandLineAgent::new()?;
            let mut is_first_round: bool = true;
            
            loop {
                let mut command_lines: Vec<&CommandLine> = Vec::new();
                
                // Use the user prompt for subsequent rounds
                if !is_first_round {
                    // Collect user prompt
                    let user_new_prompt: String = input_message(
                        "Please further instruct the LLM: "
                    )?;
                    display_message(Level::Logging, "LLM is thinking...");
                    command_lines = agent.next_step(&user_new_prompt)?;
                }
                
                // Use the user query provided in the `run` argument for the first round
                if is_first_round {
                    is_first_round = false;
                    display_message(Level::Logging, "LLM is thinking...");
                    command_lines = agent.next_step(
                        &subcommand.command_in_natural_language
                    )?;
                }
                
                let command_lines_text: String = command_lines.iter()
                    .map(|command| "    > ".to_string() + command.to_string().as_str())
                    .collect::<Vec<String>>()
                    .join("\n");
                let user_input: String = input_message(
                    &format!("Execute the following command(s)? (y/n)\n{}", command_lines_text)
                )?;
                
                if user_input.trim() == "y" {
                    match agent.execute() {
                        Ok(_) => {
                            display_message(Level::Logging, "Commands had been executed successfully.");
                            break;
                        },
                        Err(error) => {
                            display_message(Level::Error, &error.to_string());
                            continue;
                        }
                    };
                }
                
                // Add the assistant generated command to the agent's memory
                agent.add(async_openai::types::Role::Assistant, command_lines_text)?;
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
