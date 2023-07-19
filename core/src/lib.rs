use app_error::AppError;
use config::{
    check_config, create_default_config, create_manual_config, delete_config_parent, InitialConfig,
};
use constants::{app_name, config_name};
use constants::{InitPushArgs, LoadTemplateArgs, SaveTemplateArgs};
use constants::{APP_NAME, CONFIG_NAME};
use std::path::Path;

fn copy_to_dest(source: &Path, dest: &Path) -> Result<(), AppError> {
    let iterated_paths = std::fs::read_dir(source)?;

    for item in iterated_paths {
        let item = item?;
        let item_path = std::fs::canonicalize(item.path())?;
        let destination_path = dest.join(item_path.file_name().unwrap());

        if item_path.is_dir() {
            copy_dir::copy_dir(item_path, destination_path)?;
        } else {
            std::fs::copy(item_path, destination_path)?;
        }
    }

    Ok(())
}

/// This function initializes the template directory, as well as the config file.
/// This function is called when the user runs the command `init`.
/// # Arguments
///
/// * `args`: &InitPushArgs - The arguments passed to the `init` command (contains the path to the template directory)
///
/// returns: Result<(), AppError>
///
/// # Examples
///
/// ```rust,ignore
/// use std::path::PathBuf;
/// use cli::{app_name, app_version, config_name, InitPushArgs, template_path};
/// use cli::constants::{APP_NAME, APP_VERSION, CONFIG_NAME, TEMPLATE_FOLDER_NAME};
/// use cli::template_config_module::InitialConfig;
///
/// let mut path = PathBuf::from("/tmp/");
/// path.push("app");
///
/// let args = InitPushArgs {
/// path: Some(path.to_str().unwrap().to_string())
/// };
/// path.push(template_path!());
///
/// init_function(&args)?;
///
/// let config: InitialConfig = confy::load(app_name!(), config_name!())?;
///
/// assert_eq!(config.template_absolute_path, path);
/// assert_eq!(config.version, app_version!());
/// assert!(config.initialized);
/// ```
pub fn init_function(args: &InitPushArgs) -> Result<(), AppError> {
    let config = match &args.path {
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
pub fn save_template_function(args: &SaveTemplateArgs) -> Result<(), AppError> {
    let path = Path::new(&args.path);
    let name = &args.name;
    let mut config = confy::load::<InitialConfig>(app_name!(), config_name!())?;

    let overwrite = args.overwrite;

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

    config.templates.push(name.to_string());
    config.templates.sort();
    confy::store(app_name!(), config_name!(), config)?;

    Ok(())
}

pub fn load_template_function(args: &LoadTemplateArgs) -> Result<(), AppError> {
    let name = &args.name;
    let path = Path::new(&args.path).canonicalize()?;

    let config = confy::load::<InitialConfig>(app_name!(), config_name!())?;
    check_config(&config)?;

    config
        .templates
        .iter()
        .find(|&x| x == name)
        .ok_or(AppError::TemplateDoesNotExist)?;

    let source = config.template_absolute_path.join(name);

    copy_to_dest(&source, &path)?;

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
        println!("\t- {}", template);
    }

    Ok(())
}
