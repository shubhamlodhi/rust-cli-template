use clap::Parser;
use {{cli_name}}::{cli::{Cli, Commands}, commands, error::Result};

fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Configure logging based on verbosity
    setup_logging(cli.verbose)?;

    // Execute the appropriate command
    match cli.command {
        Commands::Example { name, formal } => {
            commands::example::execute(name, formal)?;
        },
        Commands::Process { input, output, mode } => {
            commands::process::execute(input, output, mode)?;
        },
    }

    Ok(())
}

fn setup_logging(verbosity: u8) -> Result<()> {
    let log_level = match verbosity {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp(None)
        .init();

    Ok(())
}
