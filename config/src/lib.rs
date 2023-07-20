use app_error::AppError;
use confy::ConfyError;
use constants::{
    app_name, app_version_string, config_name, template_default_path, template_folder_name,
    template_path,
};
use constants::{APP_NAME, APP_VERSION_STRING, CONFIG_NAME, TEMPLATE_FOLDER_NAME};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TemplateType {
    Default = 0,
    Remote = 1,
}

impl PartialEq for TemplateType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub name: String,
    pub template_type: TemplateType,
}

impl Template {
    pub fn new(name: &str, template_type: TemplateType) -> Self {
        Self {
            name: name.to_owned(),
            template_type,
        }
    }
}

impl PartialEq for Template {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.template_type == other.template_type
    }
}

impl PartialOrd for Template {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        (self.template_type as i32).partial_cmp(&(other.template_type as i32))
    }
}

impl Eq for Template {}

impl Ord for Template {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

#[macro_export]
macro_rules! template {
    ($a:expr,$b:expr) => {
        Template::new($a, $b)
    };
    ($a:expr) => {
        Template::new($a, TemplateType::Default)
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitialConfig {
    pub version: String,
    pub template_absolute_path: PathBuf,
    pub initialized: bool,
    pub templates: Vec<Template>,
}

impl InitialConfig {
    /// Creates a new InitialConfig object from a given version and template path.
    ///
    /// # Arguments
    ///
    /// * `version`:
    /// * `template_absolute_path`:
    /// * `initialized`:
    ///
    /// returns: Result<InitialConfig, Error>
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::path::Path;
    /// use cli::template_config_module::InitialConfig;
    /// use cli::{template_path, app_name, template_folder_name};
    /// use cli::constants::{TEMPLATE_FOLDER_NAME, APP_NAME};
    ///
    /// let test_version = 1.0;
    /// let mut test_template_absolute_path = Path::new("/home/user").to_path_buf();
    /// test_template_absolute_path.push(template_path!());
    /// let test_initialized = false;
    ///
    /// let version = 1;
    /// let template_absolute_path = Path::new("/home/user");
    ///
    /// let initial_config = InitialConfig::new(version as f32, template_absolute_path).unwrap();
    ///
    /// assert_eq!(initial_config.version, test_version);
    /// assert_eq!(initial_config.template_absolute_path, test_template_absolute_path);
    /// assert_eq!(initial_config.initialized, test_initialized);
    /// ```
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

    /// Creates a default InitialConfig object.
    ///
    /// # Arguments
    ///
    /// returns: Result<InitialConfig, Error>
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use cli::{app_version, template_default_path, template_folder_name, app_name};
    /// use cli::constants::{APP_VERSION, TEMPLATE_FOLDER_NAME, APP_NAME};
    /// use cli::template_config_module::InitialConfig;
    ///
    ///
    /// let initial_config = InitialConfig::default();
    /// let test_version = app_version!();
    /// let mut test_template_absolute_path = home::home_dir().unwrap();
    /// test_template_absolute_path.push(template_default_path!());
    ///
    /// assert_eq!(initial_config.version, test_version);
    /// assert_eq!(initial_config.template_absolute_path, test_template_absolute_path);
    /// assert_eq!(initial_config.initialized, false);
    /// ```
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

/// Creates a default InitialConfig object and creates the template folder if it does not exist.
///
/// # Arguments
///
/// returns: Result<InitialConfig, Error>
///
/// # Examples
///
/// ```rust,ignore
/// use cli::template_config_module::create_default_config;
/// use template_config::create_default_config;
///
/// let initial_config = create_default_config().unwrap();
///
/// let test_version = 1;
/// let mut test_template_absolute_path = home::home_dir().unwrap();
/// test_template_absolute_path.push(template_default_path!());
/// let test_initialized = true;
///
/// assert_eq!(initial_config.version, test_version);
/// assert_eq!(initial_config.template_absolute_path, test_template_absolute_path);
/// assert_eq!(initial_config.initialized, test_initialized);
/// if !initial_config.template_absolute_path.exists() {
///    panic!("Template path does not exist.");
/// }
/// ```
pub fn create_default_config() -> Result<InitialConfig, std::io::Error> {
    let mut config = InitialConfig::default_value()?;

    if !config.template_absolute_path.exists() {
        std::fs::create_dir_all(&config.template_absolute_path)?;
    }

    config.initialized = true;
    Ok(config)
}

/// Creates a new InitialConfig object from a given version and template path. Creates the template folder if it does not exist.
///
/// # Arguments
///
/// * `template_absolute_path`:
///
/// returns: Result<InitialConfig, Error>
///
/// # Examples
///
/// ```rust,ignore
/// use template_config::create_manual_config;
/// use template_config::template_path;
/// use templateR::app_version;
/// use std::path::Path;
/// use cli::{app_version, template_path};
/// use cli::template_config_module::create_manual_config;
///
/// let test_version: f32 = app_version!();
/// let mut test_template_absolute_path = Path::new("/tmp").to_path_buf();
/// test_template_absolute_path.push(template_path!());
/// let test_initialized = true;
///
/// let template_absolute_path = Path::new("/tmp");
///
/// let initial_config = create_manual_config(template_absolute_path).unwrap();
///
/// assert_eq!(initial_config.version, test_version);
/// assert_eq!(initial_config.template_absolute_path, test_template_absolute_path);
/// assert_eq!(initial_config.initialized, test_initialized);
/// ```
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

/// Checks if the given InitialConfig object is valid and if the template path exists.
///
/// # Arguments
///
/// * `config`:
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```rust,ignore
/// use cli::template_config_module::check_config;
/// use template_config::check_config;
/// use template_config::InitialConfig;
/// use template_config::template_path;
///
/// match check_config(&config) {
///    Ok(_) => println!("Config is valid."),
///   Err(error) => println!("Config is invalid: {}", error),
/// }
/// ```
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
