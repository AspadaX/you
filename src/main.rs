mod agents;
mod arguments;
mod configurations;
mod constants;
mod helpers;
mod information;
mod llm;
mod styles;

use anyhow::{Error, Result};
use arguments::{Arguments, Commands};
use cchain::display_control::{Level, display_message};
use clap::{Parser, crate_authors, crate_description, crate_name, crate_version};
use helpers::{
    process_explanation_with_one_single_instruction, process_interactive_mode,
    process_run_with_one_single_instruction,
};

use crate::{configurations::Configurations, information::ContextualInformation};

fn main() -> Result<(), Error> {
    let arguments: Arguments = Arguments::parse();

    Configurations::initialize()?;
    let contextual_information: ContextualInformation = ContextualInformation::new()?;

    match arguments.commands {
        Commands::Run(subcommand) => {
            if let Some(command_in_natural_language) = subcommand.command_in_natural_language {
                process_run_with_one_single_instruction(
                    &contextual_information,
                    &command_in_natural_language,
                )?;
                return Ok(());
            }

            process_interactive_mode(&contextual_information)?;
        }
        Commands::Explain(subcommand) => {
            process_explanation_with_one_single_instruction(&subcommand.command, &contextual_information)?;
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
