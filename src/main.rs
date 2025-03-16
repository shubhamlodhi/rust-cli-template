{% if include-logging %}
use log::{info, debug, error};
{% endif %}
use clap::Parser;
use anyhow::Result;

{% if include-config %}
mod config;
{% endif %}
mod cli;
mod error;

use cli::Cli;
use error::AppError;

fn main() -> Result<()> {
    {% if include-logging %}
    env_logger::init();
    info!("Starting {{cli-name}}");
    {% endif %}

    let cli = Cli::parse();
    {% if include-logging %}
    debug!("CLI arguments: {:?}", cli);
    {% endif %}

    match run(cli) {
        Ok(_) => {
            {% if include-logging %}
            info!("{{cli-name}} completed successfully");
            {% endif %}
            Ok(())
        },
        Err(e) => {
            {% if include-logging %}
            error!("Error: {}", e);
            {% endif %}
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn run(cli: Cli) -> Result<()> {
    {% if include-logging %}
    info!("Processing command: {:?}", cli.command);
    {% endif %}

    match cli.command {
        cli::Commands::Example { name } => {
            println!("Hello, {}!", name);
            Ok(())
        }
        // Add more commands as needed
    }
}
