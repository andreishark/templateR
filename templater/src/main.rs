use cli;
use app_error;

use clap::Parser;
use cli::{Cli, match_commands};
use app_error::AppError;

fn main() -> Result<(), AppError> {
    let local_cli: Cli = cli::Cli::parse();

    match_commands(&local_cli)?;

    Ok(())
}

