use anyhow::{Error, Result};
use cchain::display_control::{display_message, Level};
use clap::Parser;
use eai::{agent::Agent, arguments::{Arguments, Commands}};

fn main() -> Result<(), Error> {
    let arguments = Arguments::parse();
    
    match arguments.commands {
        Commands::Run(subcommand) => {
            let mut agent = Agent::new()?;
            let mut round_number: usize = 0;
            
            loop {
                // Use the user query provided in the `run` argument for the first round
                display_message(Level::Logging, "LLM is coming up commands...");
                if round_number == 0 {
                    agent.iterate_command_line_with_llm(&subcommand.command_in_natural_language)?;
                }
                
                match agent.execute() {
                    Ok(_) => {
                        display_message(Level::Logging, "Commands had been executed successfully.");
                        break;
                    },
                    Err(error) => {
                        display_message(Level::Error, &error.to_string());
                        if error.to_string() == "Execution rejected" {
                            break;
                        }
                        
                        continue;
                    }
                };
            }
        },
        Commands::Version(_) => {
            println!("done version 0.1.0");
        }
    }
    
    Ok(())
}
