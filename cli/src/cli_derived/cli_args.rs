use clap::Args;
use http::Uri;

#[derive(Debug, Args)]
pub struct InitPushArgs {
    /// Path to the template directory
    #[arg(short, long)]
    pub path: Option<String>,
}

#[derive(Debug, Args)]
pub struct SaveTemplateArgs {
    /// Name of the template
    pub name: String,
    /// Path to the template directory that you want to save
    pub path: String,
    /// Overwrite the template if it already exists
    #[arg(short, long, action)]
    pub overwrite: bool,
}

#[derive(Debug, Args)]
pub struct LoadTemplateArgs {
    /// Name of the template
    pub name: String,
    /// Path to the template directory that you want to save
    pub path: String,
}

#[derive(Debug, Args)]
pub struct RemoteGetArgs {
    /// Url to git repository
    pub url: Uri,

    /// Jump over templates that can't be copied
    #[arg(short, long)]
    pub skip_config_error: Option<bool>,
}
