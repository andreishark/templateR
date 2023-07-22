use clap::Subcommand;

use super::cli_args::RemoteGetArgs;

#[derive(Subcommand)]
pub enum InitCommands {
    Delete,
}

#[derive(Subcommand)]
pub enum ShowCommands {
    Config,
    Templates,
}

#[derive(Subcommand)]
pub enum RemoteCommands {
    Get {
        #[command(flatten)]
        args: RemoteGetArgs,
    },
    Save,
}
