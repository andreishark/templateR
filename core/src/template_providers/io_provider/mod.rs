mod io_functionality;

use app_error::AppError;
use config::{check_config, InitialConfig};
use constants::{app_name, config_name, APP_NAME, CONFIG_NAME};

use crate::template_interface::TemplateInterface;

use self::io_functionality::copy_to_dest;

pub struct IOProvider {
    config: InitialConfig,
}

impl IOProvider {
    pub fn new() -> Result<Self, AppError> {
        let config = confy::load::<InitialConfig>(app_name!(), config_name!())?;
        check_config(&config)?;

        Ok(Self { config })
    }
}

impl TemplateInterface for IOProvider {
    fn save_single(
        &self,
        template: &config::template::Template,
        overwrite: bool,
        path: &str,
    ) -> Result<(), app_error::AppError> {
        let path = std::path::Path::new(path);

        let destination = self.config.template_absolute_path.join(template.name);
        let source = path;

        std::fs::create_dir_all(&destination)?;

        copy_to_dest(source, &destination)?;

        Ok(())
    }

    fn load(&self, name: &str, path: &str) -> Result<(), app_error::AppError> {
        let path = std::path::Path::new(path);
        let absolute_path = path.canonicalize()?;

        self.config
            .templates
            .iter()
            .find(|&x| x.name.as_str() == name)
            .ok_or(AppError::TemplateDoesNotExist)?;

        let source = self.config.template_absolute_path.join(name);

        copy_to_dest(&source, &absolute_path)?;

        Ok(())
    }

    fn save_many(&self, path: &str) -> Result<(), app_error::AppError> {
        todo!()
    }
}
