mod cli_derived;
use app_error::AppError;
use cli_derived::match_commands_derived;
pub use cli_derived::Cli;

/// This command will match the commands to the corresponding functions
///
/// # Arguments
///
/// * `cli`: &Cli - The arguments passed to the `init` command (contains the path to the template directory)
///
/// returns: Result<(), AppError>
///
/// # Examples
///
/// ```rust,ignore
/// use cli::{Cli, match_commands};
/// let cli = Cli::parse();
///
/// match_commands(&cli)?;
/// ```
pub fn match_commands_derive(cli: &Cli) -> Result<(), AppError> {
    match_commands_derived(cli)?;
    Ok(())
}
