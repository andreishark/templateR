use std::{io};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("Confif parser Error: {0}")]
    Confy(#[from] confy::ConfyError),

    #[error("The template directory is not initialized. Please run `templater init` first.")]
    TemplateNotInitialized,

    #[error("The template directory is already initialized. Please run `templateR init delete` to delete.")]
    TemplateAlreadyInitialized,

    #[error("The template directory does not exist. Please run `templateR init delete` first then `templateR init` to initialize or \
    add a template with that name `templateR save-template [NAME] [PATH]`")]
    TemplateDoesNotExist,

    #[error("Template already exists. Please run `templateR save-template [NAME] [PATH] --overwrite` to overwrite.")]
    TemplateAlreadyExists,
}

// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self)?;
//         Ok(())
//     }
// }
