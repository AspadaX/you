use anyhow::{Error, Result};
use clap::Parser;
use done_rs::{agent::Agent, arguments::{Arguments, Commands}};

fn main() -> Result<(), Error> {
    let arguments = Arguments::parse();
    
    match arguments.commands {
        Commands::Run(subcommand) => {
            let mut agent = Agent::new(
                subcommand.command_in_natural_language
            )?;
            
            agent.breakdown()?;
            agent.execute()?;
            
            println!("{}", &agent)
        },
        Commands::Version(_) => {
            println!("done version 0.1.0");
        }
    }
    
    Ok(())
}
