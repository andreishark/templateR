use clap::Parser;
use cli::match_commands;
use cli::AppError;

fn main() -> Result<(), AppError> {
    let cli = cli::Cli::parse();

    match_commands(&cli)?;

    Ok(())
}

