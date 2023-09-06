pub static APP_NAME: &str = "templater";
pub static CONFIG_NAME: &str = "config";
pub static APP_VERSION_STRING: &str = env!("CARGO_PKG_VERSION");
pub static APP_AUTHOR: &str = "andreishark";
pub static APP_ABOUT: &str = "A simple templating tool.";
pub static TEMPLATE_FOLDER_NAME: &str = "templates";
pub static REMOTE_TEMPLATE_CONFIG_NAME: &str = "config.json";
pub static TEMP_FOLDER_NAME: &str = "temp";
pub static GIT_BASE_URL: &str = "https://github.com/andreishark/templater-templates.git";

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

#[macro_export]
macro_rules! temp_folder_name {
    () => {
        TEMP_FOLDER_NAME
    };
}

#[macro_export]
macro_rules! git_base_url {
    () => {
        GIT_BASE_URL
    };
}
