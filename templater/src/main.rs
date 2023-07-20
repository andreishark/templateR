use app_error;
use cli;

use app_error::AppError;
use clap::Parser;
use cli::{match_commands_derive, Cli};

fn main() -> Result<(), AppError> {
    let local_cli: Cli = cli::Cli::parse();

    match_commands_derive(&local_cli)?;

    Ok(())
}
