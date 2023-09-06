pub mod config_logic;
mod template_interface;
pub mod template_providers;

use app_error::AppError;
use config::{check_config, template, template::Template, template::TemplateType, InitialConfig};
use constants::{app_name, config_name, remote_template_config_name, temp_folder_name};
use constants::{APP_NAME, CONFIG_NAME, REMOTE_TEMPLATE_CONFIG_NAME, TEMP_FOLDER_NAME};
use git2::Repository;
use http::Uri;
use serde::{Deserialize, Serialize};
use std::fs::remove_dir_all;
use std::path::Path;
use std::process::Command;
use template_interface::TemplateInterface;

#[derive(Serialize, Deserialize)]
struct RemoteTemplateConfig {
    templates: Vec<String>,
}

pub fn save_single_template(
    template_provider: &dyn TemplateInterface,
    path: &Path,
    overwrite: bool,
    template: Template,
) -> Result<(), AppError> {
    let mut config = confy::load::<InitialConfig>(app_name!(), config_name!())?;
    check_config(&config)?;

    if config.template_absolute_path.join(template.name).exists() && !overwrite {
        return Err(AppError::TemplateAlreadyExists);
    }

    if overwrite {
        std::fs::remove_dir_all(config.template_absolute_path.join(template.name))?;
    }

    let str_path = match path.to_str() {
        Some(value) => value,
        None => return Err(AppError::InvalidPath),
    };

    template_provider.save_single(&template, overwrite, str_path)?;

    config.templates.push(template);
    config.templates.sort();
    confy::store(app_name!(), config_name!(), config)?;

    Ok(())
}

// pub fn save_many_template_function() {}

pub fn load_template(
    template_provider: &dyn TemplateInterface,
    name: &str,
    path: &Path,
) -> Result<(), AppError> {
    let config = confy::load::<InitialConfig>(app_name!(), config_name!())?;
    check_config(&config)?;

    let path_str = match path.to_str() {
        Some(value) => value,
        None => return Err(AppError::InvalidPath),
    };

    template_provider.load(name, path_str);
    Ok(())
}

pub fn show_config() -> Result<(), AppError> {
    let config = confy::load::<InitialConfig>(app_name!(), config_name!())?;
    check_config(&config)?;

    println!(
        "Config file path: {}",
        confy::get_configuration_file_path(app_name!(), config_name!())?
            .to_str()
            .unwrap()
    );
    println!("Version: {}", config.version);
    println!(
        "Template directory: {}",
        config.template_absolute_path.to_str().unwrap()
    );
    println!("Templates: ");
    for template in config.templates {
        println!("\t- {}", template.name);
    }

    Ok(())
}

pub fn show_templates() -> Result<(), AppError> {
    let config = confy::load::<InitialConfig>(app_name!(), config_name!())?;
    check_config(&config)?;

    if config.templates.is_empty() {
        println!("No templates found");
        return Ok(());
    }

    for template in config.templates {
        println!("- {}", template.name);
    }

    Ok(())
}

pub fn clone_template_from_remote(
    url: Uri,
    skip_config_error: Option<bool>,
) -> Result<(), AppError> {
    let skip_config_error = skip_config_error.unwrap_or(false);

    let config = confy::load::<InitialConfig>(app_name!(), config_name!())?;
    check_config(&config)?;
    let temp_path = config.template_absolute_path.join(temp_folder_name!());
    println!("Cloning template to {}", temp_path.display());

    Repository::clone(&url.to_string(), &temp_path)?;

    let repo_path = temp_path.as_path();

    if !repo_path.join(remote_template_config_name!()).exists() {
        return Err(AppError::TemplateInvalidConfig);
    }

    let template_config: RemoteTemplateConfig = serde_json::from_reader(std::fs::File::open(
        repo_path.join(remote_template_config_name!()),
    )?)?;

    for template in template_config.templates {
        let source = repo_path.join(&template);

        if !source.is_dir() && !skip_config_error {
            return Err(AppError::TemplateDoesNotExist);
        }

        if !source.is_dir() && skip_config_error {
            println!(
                "Template {} from config doesn't exist. Skipping...",
                template
            );
        }

        match save_template_function(&template, &source, false, Some(TemplateType::Remote)) {
            Ok(_) => {}
            Err(e) => match e {
                AppError::TemplateAlreadyExists => {
                    println!("Template {} already exists. Skipping...", template);
                    continue;
                }
                _ => return Err(e),
            },
        }
    }
    remove_dir_all(temp_path)?;

    Ok(())
}

pub fn add_new_template_git(path: &Path) -> Result<(), AppError> {
    let config = confy::load::<InitialConfig>(app_name!(), config_name!())?;
    check_config(&config)?;

    let temp_path = config.template_absolute_path.join(temp_folder_name!());
    println!("Cloning template to {}", temp_path.display());

    let repo = Repository::clone(path.to_str().unwrap(), &temp_path)?;

    let repo_path = repo.path();
    let remote_config_file_path = repo_path.join(remote_template_config_name!());

    if !repo_path.join(remote_template_config_name!()).exists() {
        return Err(AppError::TemplateInvalidConfig);
    }

    let mut template_config: RemoteTemplateConfig =
        serde_json::from_reader(std::fs::File::open(&remote_config_file_path)?)?;

    let template_name = path.file_name().unwrap().to_str().unwrap();

    if template_config
        .templates
        .binary_search(&template_name.to_owned())
        .is_ok()
    {
        return Err(AppError::TemplateAlreadyExists);
    }

    let destination = temp_path.join(path.file_name().unwrap());
    copy_to_dest(path, &destination)?;

    template_config.templates.push(template_name.to_owned());

    serde_json::to_writer(
        std::fs::File::create(&remote_config_file_path)?,
        &template_config,
    )?;

    let commit_message = format!("Add template {}", template_name);

    Command::new("git")
        .args(&[
            "add",
            destination.to_str().unwrap(),
            remote_config_file_path.to_str().unwrap(),
        ])
        .output()?;

    Command::new("git")
        .args(&["commit", "-m", commit_message.as_str()])
        .output()?;
    Command::new("git").args(&["push"]).output()?;

    println!("Template {} added successfully", template_name);
    Ok(())
}
