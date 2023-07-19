#[cfg(test)]
mod tests {
    use config::{check_config, create_default_config, create_manual_config, InitialConfig};
    use constants::{
        app_name, app_version_string, template_default_path, template_folder_name, template_path,
    };
    use constants::{APP_NAME, APP_VERSION_STRING, TEMPLATE_FOLDER_NAME};
    use std::path::Path;

    #[test]
    fn test_initial_config_new() {
        let test_version: &str = "1.0";
        let mut test_template_absolute_path = Path::new("/home/user").to_path_buf();
        test_template_absolute_path.push(template_path!());
        let test_initialized = false;

        let version: &str = "1.0";
        let template_absolute_path = Path::new("/home/user");

        let initial_config = InitialConfig::new(version, template_absolute_path).unwrap();

        assert_eq!(initial_config.version, test_version);
        assert_eq!(
            initial_config.template_absolute_path,
            test_template_absolute_path
        );
        assert_eq!(initial_config.initialized, test_initialized);
    }

    #[test]
    fn test_initial_config_default() {
        let initial_config = InitialConfig::default_value().unwrap();
        let test_version = app_version_string!();
        let mut test_template_absolute_path = home::home_dir().unwrap();
        test_template_absolute_path.push(template_default_path!());

        assert_eq!(initial_config.version, test_version);
        assert_eq!(
            initial_config.template_absolute_path,
            test_template_absolute_path
        );
        assert!(!initial_config.initialized)
    }

    #[test]
    fn test_create_default_value_config() {
        let initial_config = create_default_config().unwrap();
        let test_version = app_version_string!();
        let mut test_template_absolute_path = home::home_dir().unwrap();
        test_template_absolute_path.push(template_default_path!());
        let test_initialized = true;

        println!("Template path: {:?}", test_template_absolute_path);
        if !test_template_absolute_path.exists() {
            panic!("Template path does not exist.");
        }

        assert_eq!(initial_config.version, test_version);
        assert_eq!(
            initial_config.template_absolute_path,
            test_template_absolute_path
        );
        assert_eq!(initial_config.initialized, test_initialized)
    }

    #[test]
    fn test_create_manual_config() {
        let test_version: &str = app_version_string!();
        let mut test_template_absolute_path = Path::new("/tmp").to_path_buf();
        test_template_absolute_path.push(template_path!());
        let test_initialized = true;

        let template_absolute_path = Path::new("/tmp");

        let initial_config = create_manual_config(template_absolute_path).unwrap();

        assert_eq!(initial_config.version, test_version);
        assert_eq!(
            initial_config.template_absolute_path,
            test_template_absolute_path
        );
        assert_eq!(initial_config.initialized, test_initialized)
    }

    #[test]
    fn test_check_config() {
        let test_version: &str = app_version_string!();
        let mut test_template_absolute_path = Path::new("/tmp").to_path_buf();
        test_template_absolute_path.push(template_path!());
        let test_initialized = true;

        let template_absolute_path = Path::new("/tmp");

        let initial_config = create_manual_config(template_absolute_path).unwrap();

        assert_eq!(initial_config.version, test_version);
        assert_eq!(
            initial_config.template_absolute_path,
            test_template_absolute_path
        );
        assert_eq!(initial_config.initialized, test_initialized);

        match check_config(&initial_config) {
            Ok(x) => x,
            Err(error) => panic!("Config is invalid: {:?}", error),
        }
    }
}
