use clap::{
    Args, Parser, Subcommand,
    builder::{
        Styles,
        styling::{AnsiColor, Effects},
    },
    crate_authors, crate_description, crate_version,
};

// Configures Clap v3-style help menu colors
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Debug, Parser)]
#[command(name = "you", author = crate_authors!(), long_version = crate_version!())]
#[command(about = crate_description!())]
#[command(styles = STYLES)]
pub struct Arguments {
    /// Grouped features provided by `you`
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run a command that is described in natural langauge.
    /// The LLM will breakdown the task and executes them.
    #[clap(short_flag = 'r')]
    Run(RunArguments),
    /// Explain a given command
    #[clap(short_flag = 'e')]
    Explain(ExplainArguments),
    /// List all saved scripts in the cache. `ls` for short
    #[clap(alias = "ls")]
    List(ListArguments),
    /// Remove a specified script from the cache. `rm` for short
    #[clap(alias = "rm")]
    Remove(RemoveArguments),
    /// Display the version of `you`
    #[clap(short_flag = 'v')]
    Version(VersionArguments),
}

#[derive(Debug, Args)]
#[command(group = clap::ArgGroup::new("sources").required(false).multiple(false))]
pub struct RunArguments {
    /// Convert natural language instruction to an executable command. Leave it empty to run interactive mode.
    #[arg(group = "sources")]
    pub command_in_natural_language: Option<String>,
}

#[derive(Debug, Args)]
#[command(group = clap::ArgGroup::new("sources").required(true).multiple(false))]
pub struct ExplainArguments {
    /// Convert natural language instruction to an executable command. Leave it empty to run interactive mode.
    #[arg(group = "sources")]
    pub command: String,
}

#[derive(Debug, Args)]
#[command(group = clap::ArgGroup::new("sources").required(false).multiple(false))]
pub struct ListArguments;

#[derive(Debug, Args)]
#[command(group = clap::ArgGroup::new("sources").required(true).multiple(false))]
pub struct RemoveArguments {
    /// Name of the script to remove from cache
    #[arg(group = "sources")]
    pub script_name: String,
}

#[derive(Debug, Args)]
#[command(group = clap::ArgGroup::new("sources").required(false).multiple(false))]
pub struct VersionArguments;
