use app_error::AppError;
use config::template::Template;

pub trait TemplateInterface {
    fn save_single(&self, template: &Template, overwrite: bool, path: &str)
        -> Result<(), AppError>;
    fn save_many(&self, path: &str) -> Result<(), AppError>;
    fn load(&self, name: &str, path: &str) -> Result<(), AppError>;
}
