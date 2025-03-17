use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "{{cli_name}}")]
#[command(author = "{{author_name}} <{{author_email}}>")]
#[command(version)]
#[command(about = "{{project_description}}", long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<String>,

    /// Turn on verbose output (use multiple times for more verbosity)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Example command that greets a person
    Example {
        /// Name of the person to greet
        #[arg(short, long)]
        name: String,
        
        /// Use formal greeting
        #[arg(short, long)]
        formal: bool,
    },
    
    /// Process a file with specified options
    Process {
        /// Input file to process
        #[arg(short, long, value_name = "FILE")]
        input: String,
        
        /// Output file (defaults to stdout if not provided)
        #[arg(short, long, value_name = "FILE")]
        output: Option<String>,
        
        /// Processing mode
        #[arg(short, long, value_enum, default_value = "normal")]
        mode: ProcessingMode,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ProcessingMode {
    Normal,
    Fast,
    Thorough,
}