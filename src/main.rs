mod agents;
mod arguments;
mod helpers;
mod information;
mod llm;
mod styles;

use anyhow::{Error, Result};
use arguments::{Arguments, Commands};
use cchain::display_control::{Level, display_message};
use clap::{Parser, crate_authors, crate_description, crate_name, crate_version};
use helpers::{process_interactive_mode, process_run_with_one_single_instruction};

fn main() -> Result<(), Error> {
    let arguments = Arguments::parse();

    match arguments.commands {
        Commands::Run(subcommand) => {
            if let Some(command_in_natural_language) = subcommand.command_in_natural_language {
                process_run_with_one_single_instruction(&command_in_natural_language)?;
                return Ok(());
            }

            process_interactive_mode()?;
        }
        Commands::Explain(subcommand) => {}
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
