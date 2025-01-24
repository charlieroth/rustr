use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub cmd: CliCommand,
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    Post { message: String },
}
