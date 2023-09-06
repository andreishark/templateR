pub mod template;

use app_error::AppError;
use confy::ConfyError;
use constants::{
    app_name, app_version_string, config_name, template_default_path, template_folder_name,
    template_path,
};
use constants::{APP_NAME, APP_VERSION_STRING, CONFIG_NAME, TEMPLATE_FOLDER_NAME};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use template::Template;

#[derive(Serialize, Deserialize, Debug)]
pub struct InitialConfig {
    pub version: String,
    pub template_absolute_path: PathBuf,
    pub initialized: bool,
    pub templates: Vec<Template>,
}

impl InitialConfig {
    pub fn new(version: &str, template_absolute_path: &Path) -> Result<Self, std::io::Error> {
        let mut template_absolute_path = template_absolute_path.to_path_buf();
        template_absolute_path.push(template_path!());

        Ok(Self {
            version: version.to_owned(),
            template_absolute_path,
            initialized: false,
            templates: Vec::new(),
        })
    }

    pub fn default_value() -> Result<Self, std::io::Error> {
        let mut home_dir = match home::home_dir() {
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Home directory not found. Specify template path manually.",
                ))
            }
            Some(value) => value,
        };
        home_dir.push(template_default_path!());

        Ok(Self {
            version: app_version_string!().to_owned(),
            template_absolute_path: home_dir,
            initialized: false,
            templates: Vec::new(),
        })
    }
}

impl Default for InitialConfig {
    fn default() -> Self {
        match Self::default_value() {
            Ok(value) => value,
            Err(error) => panic!("Error: {}", error),
        }
    }
}

pub fn create_default_config() -> Result<InitialConfig, std::io::Error> {
    let mut config = InitialConfig::default_value()?;

    if !config.template_absolute_path.exists() {
        std::fs::create_dir_all(&config.template_absolute_path)?;
    }

    config.initialized = true;
    Ok(config)
}

pub fn create_manual_config(
    template_absolute_path: &Path,
) -> Result<InitialConfig, std::io::Error> {
    let mut config = InitialConfig::new(app_version_string!(), template_absolute_path)?;

    if !config.template_absolute_path.exists() {
        std::fs::create_dir_all(&config.template_absolute_path)?;
    }

    config.initialized = true;
    Ok(config)
}

pub fn delete_config_parent() -> Result<(), AppError> {
    let config_path = confy::get_configuration_file_path(app_name!(), config_name!())?;

    let parent = match config_path.parent() {
        None => {
            return Err(AppError::Confy(ConfyError::BadConfigDirectory(
                String::from("The toml doesn't have a parent folder"),
            )));
        }
        Some(path) => path,
    };

    if !parent.exists() {
        return Ok(());
    }

    std::fs::remove_dir_all(parent)?;

    Ok(())
}

pub fn check_config(config: &InitialConfig) -> Result<(), AppError> {
    if !config.initialized {
        delete_config_parent()?;
        return Err(AppError::TemplateNotInitialized);
    }

    if !config.template_absolute_path.exists() {
        delete_config_parent()?;
        return Err(AppError::TemplateNotInitialized);
    }

    Ok(())
}
