mod agents;
mod arguments;
mod helpers;
mod information;
mod llm;
mod styles;
mod configurations;
mod constants;

use anyhow::{Error, Result};
use arguments::{Arguments, Commands};
use cchain::display_control::{Level, display_message};
use clap::{Parser, crate_authors, crate_description, crate_name, crate_version};
use helpers::{process_explanation_with_one_single_instruction, process_interactive_mode, process_run_with_one_single_instruction};

use crate::configurations::Configurations;

fn main() -> Result<(), Error> {
    let arguments: Arguments = Arguments::parse();
    
    Configurations::initialize()?;
    let configurations: Configurations = Configurations::load()?;

    match arguments.commands {
        Commands::Run(subcommand) => {
            if let Some(command_in_natural_language) = subcommand.command_in_natural_language {
                process_run_with_one_single_instruction(&configurations, &command_in_natural_language)?;
                return Ok(());
            }

            process_interactive_mode(&configurations)?;
        }
        Commands::Explain(subcommand) => {
            process_explanation_with_one_single_instruction(&subcommand.command)?;
        }
        Commands::Version(_) => {
            display_message(Level::Logging, &format!("{}", crate_name!()));
            display_message(Level::Logging, &format!("version.{}", crate_version!()));
            display_message(Level::Logging, &format!("Authors: {}", crate_authors!()));
            display_message(
                Level::Logging,
                &format!("Description: {}", crate_description!()),
            );
        }
    }

    Ok(())
}
