use std::path::Path;

use app_error::AppError;
use config::template::Template;

pub trait TemplateInterface {
    fn save(template: Template, overwrite: bool, path: &str) -> Result<(), AppError>;
    fn load_single(path: &str) -> Result<(), AppError>;
    fn load_many(path: &str) -> Result<(), AppError>;
}
