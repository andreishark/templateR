pub static APP_NAME: &str = "templater";
pub static CONFIG_NAME: &str = "config";
pub static APP_VERSION: f32 = 1.0;
pub static APP_VERSION_STRING: &str = "1.0";
pub static APP_AUTHOR: &str = "andreishark";
pub static APP_ABOUT: &str = "A simple templating tool.";

#[macro_export]
macro_rules! app_name {
    () => {
        APP_NAME
    };
}

#[macro_export]
macro_rules! app_version {
    () => {
        APP_VERSION
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
