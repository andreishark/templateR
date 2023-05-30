use clap::Parser;
use crate::app_error::AppError;
use crate::cli::{Commands, match_commands};
use crate::template_config::InitialConfig;

mod app_error;
mod template_config;
mod cli;
mod constants;

fn main() -> Result<(), AppError> {
    let cli = cli::Cli::parse();

    match_commands(&cli)?;

    Ok(())
}
