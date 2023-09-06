use std::path::Path;

use app_error::AppError;

pub fn copy_to_dest(source: &Path, dest: &Path) -> Result<(), AppError> {
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
