mod agents;
mod arguments;
mod cache;
mod configurations;
mod constants;
mod helpers;
mod information;
mod llm;
mod styles;
mod traits;
mod shell;

use std::{fs::File, io::Read};

use anyhow::{Error, Result};
use arguments::{Arguments, Commands};
use cchain::display_control::{Level, display_message};
use clap::{Parser, crate_authors, crate_description, crate_name, crate_version};
use helpers::{
    process_explanation_with_one_single_instruction, process_interactive_mode,
    process_run_with_one_single_instruction,
};

use crate::{
    cache::Cache, configurations::Configurations, information::ContextualInformation, traits::GlobalResourceInitialization
};

fn main() -> Result<(), Error> {
    let arguments: Arguments = Arguments::parse();

    Configurations::initialize()?;
    Cache::initialize()?;
    let contextual_information: ContextualInformation = ContextualInformation::new()?;
    let cache: Cache = Cache::load()?;

    match arguments.commands {
        Commands::Run(subcommand) => {
            if let Some(command_in_natural_language) = subcommand.command_in_natural_language {
                if let Some(script) = cache.search(command_in_natural_language) {
                    let script_content = File::open(script)?.read_to_string(buf);
                }
                
                process_run_with_one_single_instruction(
                    &contextual_information,
                    &command_in_natural_language,
                )?;
                return Ok(());
            }

            process_interactive_mode(&contextual_information)?;
        }
        Commands::Explain(subcommand) => {
            process_explanation_with_one_single_instruction(
                &subcommand.command,
                &contextual_information,
            )?;
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
