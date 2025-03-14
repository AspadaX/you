use anyhow::{Error, Result};
use cchain::display_control::{display_message, Level};
use clap::Parser;
use eai::{agent::Agent, arguments::{Arguments, Commands}};

fn main() -> Result<(), Error> {
    let arguments = Arguments::parse();
    
    match arguments.commands {
        Commands::Run(subcommand) => {
            let mut agent = Agent::new(
                subcommand.command_in_natural_language
            )?;
            
            loop {
                display_message(Level::Logging, "LLM is coming up commands...");
                agent.breakdown()?;
                
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
