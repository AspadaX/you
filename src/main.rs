mod agents;
mod arguments;
mod cache;
mod configurations;
mod constants;
mod helpers;
mod information;
mod llm;
mod shell;
mod styles;
mod traits;

use std::{fs::File, io::Read};

use anyhow::{Error, Result};
use arguments::{Arguments, Commands};
use cchain::display_control::{Level, display_message};
use clap::{Parser, crate_authors, crate_description, crate_name, crate_version};
use helpers::{
    process_explanation_with_one_single_instruction, process_interactive_mode,
    process_list_cached_scripts, process_remove_cached_script,
    process_run_with_one_single_instruction,
};

use crate::{
    cache::Cache, configurations::Configurations, information::ContextualInformation,
    shell::execute_shell_script, traits::GlobalResourceInitialization,
};

fn main() -> Result<(), Error> {
    let arguments: Arguments = Arguments::parse();

    Configurations::initialize()?;
    Cache::initialize()?;

    let mut cache: Cache = Cache::load()?;
    let contextual_information: ContextualInformation = ContextualInformation::new()?;
    let configurations: Configurations = Configurations::load()?;

    match arguments.commands {
        Commands::Run(subcommand) => {
            if let Some(command_in_natural_language) = subcommand.command_in_natural_language {
                if configurations.enable_cache {
                    display_message(Level::Logging, "Cache has been enabled.");
                    if let Some(script) = cache.search(&command_in_natural_language) {
                        display_message(
                            Level::Logging,
                            &format!(
                                "Cache hit. Using the generated script {}...",
                                script.file_name().unwrap().to_str().unwrap()
                            ),
                        );
                        let mut script_content: String = String::new();
                        File::open(script)?.read_to_string(&mut script_content)?;
                        execute_shell_script(&script_content)?;
                        return Ok(());
                    }
                }

                process_run_with_one_single_instruction(
                    &mut cache,
                    &configurations,
                    &contextual_information,
                    &command_in_natural_language,
                )?;
                return Ok(());
            }

            process_interactive_mode(&mut cache, &configurations, &contextual_information)?;
        }
        Commands::Explain(subcommand) => {
            process_explanation_with_one_single_instruction(
                &subcommand.command,
                &contextual_information,
            )?;
        }
        Commands::List(_) => {
            process_list_cached_scripts(&cache)?;
        }
        Commands::Remove(subcommand) => {
            process_remove_cached_script(&mut cache, &subcommand.script_name)?;
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
