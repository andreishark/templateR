use app_error::AppError;
use clap::{Parser, Subcommand};
use constants::{
    app_about, app_author, app_name, app_version_string, InitPushArgs, LoadTemplateArgs,
    SaveTemplateArgs,
};
use constants::{APP_ABOUT, APP_AUTHOR, APP_NAME, APP_VERSION_STRING};
use core::{
    delete_init_function, init_function, load_template_function, save_template_function,
    show_config,
};

#[derive(Subcommand)]
pub enum InitCommands {
    Delete,
}

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
    SaveTemplate {
        #[command(flatten)]
        save: SaveTemplateArgs,
    },
    #[command(arg_required_else_help = true)]
    LoadTemplate {
        #[command(flatten)]
        load: LoadTemplateArgs,
    },
    ShowConfig,
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
pub fn match_commands(cli: &Cli) -> Result<(), AppError> {
    match &cli.command {
        Commands::Init { push, command } => {
            match command {
                None => init_function(push)?,
                Some(commands) => match commands {
                    InitCommands::Delete => delete_init_function()?,
                },
            };
        }

        Commands::SaveTemplate { save } => save_template_function(save)?,
        Commands::LoadTemplate { load } => load_template_function(load)?,
        Commands::ShowConfig => show_config()?,
    }
    Ok(())
}
