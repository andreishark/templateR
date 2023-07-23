mod cli_args;
mod cli_subcommands;
mod handlers;

use app_error::AppError;
use clap::{Parser, Subcommand};
use constants::{app_about, app_author, app_name, app_version_string};
use constants::{APP_ABOUT, APP_AUTHOR, APP_NAME, APP_VERSION_STRING};
use core::{
    clone_template_from_remote, delete_init_function, init_function, load_template_function,
    save_template_function, show_config, show_templates,
};

use std::path::Path;

use self::cli_args::{InitPushArgs, LoadTemplateArgs, SaveTemplateArgs};

use self::cli_subcommands::{InitCommands, RemoteCommands, ShowCommands};

#[derive(Parser)]
#[command(name = app_name!(), version = app_version_string!(), author = app_author!())]
#[command(about = app_about!())]
#[command(propagate_version = true)]
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
        push: InitPushArgs,
    },
    #[command(arg_required_else_help = true)]
    Save {
        #[command(flatten)]
        save: SaveTemplateArgs,
    },
    #[command(arg_required_else_help = true)]
    Load {
        #[command(flatten)]
        load: LoadTemplateArgs,
    },
    Show {
        #[command(subcommand)]
        command: ShowCommands,
    },
    Remote {
        #[command(subcommand)]
        command: RemoteCommands,
    },
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
/// ```rust,ignore
/// use cli::{Cli, match_commands};
/// let cli = Cli::parse();
///
/// match_commands(&cli)?;
/// ```
pub fn match_commands_derived(cli: &Cli) -> Result<(), AppError> {
    match &cli.command {
        Commands::Init { push, command } => {
            match command {
                None => init_function(&push.path)?,
                Some(commands) => match commands {
                    InitCommands::Delete => delete_init_function()?,
                },
            };
        }

        Commands::Save { save } => {
            save_template_function(&save.name, Path::new(&save.path), save.overwrite, None)?
        }
        Commands::Load { load } => load_template_function(&load.name, Path::new(&load.path))?,
        Commands::Show { command } => match command {
            ShowCommands::Config => show_config()?,
            ShowCommands::Templates => show_templates()?,
        },
        Commands::Remote { command } => match command {
            RemoteCommands::Get { args } => {
                let result = clone_template_from_remote(args.url.clone(), args.skip_config_error);
                handlers::handle_remote_get_error(result)?;
            }
            RemoteCommands::Save => todo!(),
        },
    }
    Ok(())
}
