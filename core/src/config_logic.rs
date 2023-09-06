use std::path::Path;

use app_error::AppError;
use config::{
    check_config, create_default_config, create_manual_config, delete_config_parent, InitialConfig,
};
use constants::{app_name, config_name, APP_NAME, CONFIG_NAME};

pub fn init_function(init_path: &Option<String>) -> Result<(), AppError> {
    let config = match init_path {
        None => create_default_config()?,
        Some(path) => create_manual_config(Path::new(&path))?,
    };

    confy::store(app_name!(), config_name!(), config)?;

    Ok(())
}

pub fn delete_init_function() -> Result<(), AppError> {
    let config = confy::load::<InitialConfig>(app_name!(), config_name!())?;

    check_config(&config)?;

    std::fs::remove_dir_all(config.template_absolute_path)?;

    delete_config_parent()?;

    Ok(())
}
