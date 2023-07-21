use clap::Args;

pub static APP_NAME: &str = env!("CARGO_PKG_NAME");
pub static CONFIG_NAME: &str = "config";
pub static APP_VERSION_STRING: &str = env!("CARGO_PKG_VERSION");
pub static APP_AUTHOR: &str = "andreishark";
pub static APP_ABOUT: &str = "A simple templating tool.";
pub static TEMPLATE_FOLDER_NAME: &str = "templates";
pub static REMOTE_TEMPLATE_CONFIG_NAME: &str = "config.json";

#[macro_export]
macro_rules! app_name {
    () => {
        APP_NAME
    };
}

#[macro_export]
macro_rules! config_name {
    () => {
        CONFIG_NAME
    };
}

#[macro_export]
macro_rules! app_author {
    () => {
        APP_AUTHOR
    };
}

#[macro_export]
macro_rules! app_version_string {
    () => {
        APP_VERSION_STRING
    };
}

#[macro_export]
macro_rules! app_about {
    () => {
        APP_ABOUT
    };
}

#[macro_export]
macro_rules! template_folder_name {
    () => {
        TEMPLATE_FOLDER_NAME
    };
}

#[macro_export]
macro_rules! template_path {
    () => {
        format!("{}/{}", app_name!(), template_folder_name!())
    };
}

#[macro_export]
macro_rules! template_default_path {
    () => {
        format!(".config/{}/{}", app_name!(), template_folder_name!())
    };
}

#[macro_export]
macro_rules! remote_template_config_name {
    () => {
        REMOTE_TEMPLATE_CONFIG_NAME
    };
}

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
