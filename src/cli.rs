use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "{{cli-name}}")]
#[command(author = "{{author-name}} <{{author-email}}>")]
#[command(version)]
#[command(about = "{{project-description}}", long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<String>,

    /// Turn on verbose output
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Example command
    Example {
        /// Name to greet
        #[arg(short, long)]
        name: String,
    },
    // Add more commands here
}