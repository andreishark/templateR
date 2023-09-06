pub mod config_logic;
mod io_functionality;
mod template_interface;

use app_error::AppError;
use config::{check_config, template, template::Template, template::TemplateType, InitialConfig};
use constants::{app_name, config_name, remote_template_config_name, temp_folder_name};
use constants::{APP_NAME, CONFIG_NAME, REMOTE_TEMPLATE_CONFIG_NAME, TEMP_FOLDER_NAME};
use git2::Repository;
use http::Uri;
use io_functionality::copy_to_dest;
use serde::{Deserialize, Serialize};
use std::fs::remove_dir_all;
use std::path::Path;
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct RemoteTemplateConfig {
    templates: Vec<String>,
}

/// This function saves a template to the template directory.
///
/// # Arguments
///
/// * `args`: A SaveTemplateArgs object
/// (name of the template, path to the template that needs saving, overwrite the template if it already exists)
///
/// returns: Result<(), AppError>
///
/// # Examples
///
/// ```rust,ignore
/// use cli::SaveTemplateArgs;
///
/// let args = SaveTemplateArgs {
///     name: "test".to_string(),
///     path: "/tmp/app".to_string(),
///     overwrite: true
/// };
///
/// save_template_function(&args)?;
///
/// ```
pub fn save_template_function(
    name: &str,
    path: &Path,
    overwrite: bool,
    template_type: Option<TemplateType>,
) -> Result<(), AppError> {
    // let path = Path::new(&args.path);
    // let name = &args.name;
    let template_type = template_type.unwrap_or(TemplateType::Default);
    let mut config = confy::load::<InitialConfig>(app_name!(), config_name!())?;

    // let overwrite = args.overwrite;

    check_config(&config)?;

    if config.template_absolute_path.join(name).exists() && !overwrite {
        return Err(AppError::TemplateAlreadyExists);
    }

    if overwrite {
        std::fs::remove_dir_all(config.template_absolute_path.join(name))?;
    }

    let destination = config.template_absolute_path.join(name);
    let source = path;

    std::fs::create_dir_all(&destination)?;

    copy_to_dest(source, &destination)?;
    config.templates.push(template!(name, template_type));
    config.templates.sort();
    confy::store(app_name!(), config_name!(), config)?;

    Ok(())
}

pub fn load_template_function(name: &str, path: &Path) -> Result<(), AppError> {
    let absolute_path = path.canonicalize()?;

    let config = confy::load::<InitialConfig>(app_name!(), config_name!())?;
    check_config(&config)?;

    config
        .templates
        .iter()
        .find(|&x| x.name.as_str() == name)
        .ok_or(AppError::TemplateDoesNotExist)?;

    let source = config.template_absolute_path.join(name);

    copy_to_dest(&source, &absolute_path)?;

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
