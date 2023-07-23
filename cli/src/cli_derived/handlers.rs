use std::fs::remove_dir_all;

use app_error::AppError;
use config::InitialConfig;
use constants::{app_name, config_name, temp_folder_name};
use constants::{APP_NAME, CONFIG_NAME, TEMP_FOLDER_NAME};

pub fn handle_remote_get_error(result: Result<(), AppError>) -> Result<(), AppError> {
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            let config = confy::load::<InitialConfig>(app_name!(), config_name!())?;
            let path = config.template_absolute_path.join(temp_folder_name!());

            if !path.exists() {
                return Err(e);
            }

            remove_dir_all(path)?;

            return Err(e);
        }
    }
}
