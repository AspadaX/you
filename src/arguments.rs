use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    }, crate_authors, crate_version, crate_description, Args, Parser, Subcommand
};

// Configures Clap v3-style help menu colors
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Debug, Parser)]
#[command(name = "done", author = crate_authors!(), long_version = crate_version!())]
#[command(about = crate_description!())]
#[command(styles = STYLES)]
pub struct Arguments {
    /// Grouped features provided by `done`
    #[clap(subcommand)]
    pub commands: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run a command that is described in natural langauge. 
    /// The LLM will breakdown the task and executes them.
    Run(RunArguments),
    /// Display the version of `done`
    Version(VersionArguments),
}

#[derive(Debug, Args)]
#[command(group = clap::ArgGroup::new("sources").required(true).multiple(false))]
pub struct RunArguments {
    /// Command or commands in natural language
    #[arg(group = "sources")]
    pub command_in_natural_language: String,
}

#[derive(Debug, Args)]
#[command(group = clap::ArgGroup::new("sources").required(false).multiple(false))]
pub struct VersionArguments;
