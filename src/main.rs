use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cli-app")]
#[command(about = "A template for a Rust CLI app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Example {
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Example { name } => {
            println!("Hello, {}!", name);
        }
    }
}
