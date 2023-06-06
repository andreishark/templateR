use clap::Parser;
use cli::{Cli, match_commands};
use cli::AppError;

fn main() -> Result<(), AppError> {
    let local_cli: Cli = cli::Cli::parse();

    match_commands(&local_cli)?;

    Ok(())
}

