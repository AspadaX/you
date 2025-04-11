mod information;
mod arguments;
mod llm;
mod agent;
mod styles;
mod helpers;

use anyhow::{Error, Result};
use cchain::display_control::{display_message, Level};
use clap::{Parser, crate_version, crate_authors, crate_description, crate_name};
use arguments::{Arguments, Commands};
use helpers::{process_interactive_mode, process_run_with_one_single_instruction};

fn main() -> Result<(), Error> {
    let arguments = Arguments::parse();
    
    match arguments.commands {
        Commands::Run(subcommand) => {
            if let Some(command_in_natural_language) = subcommand.command_in_natural_language {
                process_run_with_one_single_instruction(&command_in_natural_language)?;
            }
            
            process_interactive_mode()?;
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
