use std::{process::Command, str::FromStr};

use app_error::AppError;
use config::{
    check_config,
    template::{self, Template},
    InitialConfig,
};
use constants::{app_name, config_name, temp_folder_name, APP_NAME, CONFIG_NAME, TEMP_FOLDER_NAME};
use git2::Repository;
use http::Uri;
use tempdir::TempDir;

use crate::template_interface::TemplateInterface;

use super::io_provider::IOProvider;

struct GitProvider {
    config: InitialConfig,
    io_provider: IOProvider,
    base_url: Uri,
}

impl GitProvider {
    pub fn new(io_provider: IOProvider) -> Result<Self, AppError> {
        let config = confy::load::<InitialConfig>(app_name!(), config_name!())?;
        check_config(&config)?;

        let base_url = Uri::from_str(&config.git_base_url)?;

        Ok(Self {
            config,
            io_provider,
            base_url,
        })
    }

    pub fn clone_to_temp(&self, path: &str) -> Result<(TempDir), AppError> {
        let temp_dir = TempDir::new(temp_folder_name!())?;

        Command::new("git")
            .args(&["clone", temp_dir.path().to_str().unwrap()])
            .spawn()?;

        Ok((temp_dir))
    }
}

impl TemplateInterface for GitProvider {
    fn save_single(
        &self,
        template: &config::template::Template,
        overwrite: bool,
        path: &str,
    ) -> Result<(), AppError> {
        let temp_dir = self.clone_to_temp(path)?;

        let str_path = match temp_dir.path().to_str() {
            Some(value) => value,
            None => return Err(AppError::InvalidPath),
        };

        self.io_provider
            .save_single(template, overwrite, str_path)?;

        Ok(())
    }

    fn save_many(&self, path: &str) -> Result<(), AppError> {
        todo!()
    }

    fn load(&self, name: &str, path: &str) -> Result<(), AppError> {
        todo!()
    }
}
