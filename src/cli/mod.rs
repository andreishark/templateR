mod functionality;

use clap::{Args, Parser, Subcommand};
use crate::{app_name, app_version_string, app_author, app_about};
use crate::app_error::AppError;
use crate::cli::functionality::{delete_init_function, init_function, load_template_function, save_template_function};
use crate::constants::{APP_NAME, APP_VERSION_STRING, APP_AUTHOR, APP_ABOUT};

#[derive(Debug, Args)]
pub struct InitPushArgs {
    /// Path to the template directory
    #[arg(short, long)]
    pub path: Option<String>
}

#[derive(Debug, Args)]
pub struct SaveTemplateArgs {
    /// Name of the template
    pub name: String,
    /// Path to the template directory that you want to save
    pub path: String,
    /// Overwrite the template if it already exists
    #[arg(short, long, action)]
    pub overwrite: bool
}

#[derive(Debug, Args)]
pub struct LoadTemplateArgs {
    /// Name of the template
    pub name: String,
    /// Path to the template directory that you want to save
    pub path: String,
}

#[derive(Subcommand)]
pub enum InitCommands {
    Delete,
}

#[derive(Parser)]
#[command(name = app_name!(), version = app_version_string!(), author = app_author!())]
#[command(about = app_about!())]
#[command(propagate_version=true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[command(subcommand)]
        command: Option<InitCommands>,

        #[command(flatten)]
        push: InitPushArgs
    },
    SaveTemplate {
        #[command(flatten)]
        save: SaveTemplateArgs
    },
    LoadTemplate {
        #[command(flatten)]
        load: LoadTemplateArgs
    }
}

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
/// ```
/// let cli = Cli::parse();
///
/// match_commands(&cli)?;
/// ```
pub fn match_commands(cli: &Cli) -> Result<(), AppError> {
    match &cli.command {
        Commands::Init { push, command } => {
            match command {
                None => init_function(push)?,
                Some(commands) => match commands {
                    InitCommands::Delete => { delete_init_function()? } }
                ,
            };
        }

        Commands::SaveTemplate { save } => { save_template_function(save)? }
        Commands::LoadTemplate { load } => { load_template_function(load)? }
    }
    Ok(())
}